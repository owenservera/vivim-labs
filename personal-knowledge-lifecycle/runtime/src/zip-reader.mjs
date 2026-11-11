// Bounded, read-only ZIP reader for explicitly selected local archives.
// Supports stored/deflated single-disk ZIP32 and bounded ZIP64; rejects encryption,
// unsafe paths, symlinks, ambiguous names, bad CRC and decompression bombs.
import { open, lstat } from 'node:fs/promises';
import { createReadStream } from 'node:fs';
import { PassThrough, Readable, Transform } from 'node:stream';
import { createInflateRaw } from 'node:zlib';
import { createHash } from 'node:crypto';

export const ZIP_LIMITS = Object.freeze({
  maxEntries: 100000, maxDirectoryBytes: 64 * 1024 * 1024,
  maxEntryBytes: 16 * 1024 * 1024 * 1024, maxTotalBytes: 64 * 1024 * 1024 * 1024,
  maxExpansionRatio: 500,
});
const fail = (code, cause) => Object.assign(new Error(code), { code, cause });
const signature = (b, i, s) => i >= 0 && i + 4 <= b.length && b.readUInt32LE(i) === s;
const u16 = (b, i) => b.readUInt16LE(i);
const u32 = (b, i) => b.readUInt32LE(i);
const safeU64 = (b, i) => {
  if (i + 8 > b.length) throw fail('ZIP64_TRUNCATED');
  const n = b.readBigUInt64LE(i);
  if (n > BigInt(Number.MAX_SAFE_INTEGER)) throw fail('ZIP64_RANGE');
  return Number(n);
};
const decoder = new TextDecoder('utf-8', { fatal: true });
function decodeName(buffer) {
  try { return decoder.decode(buffer); }
  catch { throw fail('ZIP_FILENAME_ENCODING'); }
}
function safeName(name) {
  if (!name || name.length > 1024 || name.includes('\\') || name.startsWith('/') || name.includes('\0') || /[\x00-\x1f]/.test(name) || /^[a-z]:/i.test(name) || name.replace(/\/$/, '').split('/').some(s => s === '..' || s === '.' || !s)) throw fail('ZIP_UNSAFE_PATH');
  return name;
}
const table = (() => {
  const values = new Uint32Array(256);
  for (let i = 0; i < 256; i++) { let c = i; for (let j = 0; j < 8; j++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; values[i] = c >>> 0; }
  return values;
})();
export function crc32Update(value, chunk) {
  let c = value >>> 0;
  for (const byte of chunk) c = table[(c ^ byte) & 0xff] ^ (c >>> 8);
  return c >>> 0;
}

export async function openZipArchive(filePath, limits = {}) {
  const cfg = { ...ZIP_LIMITS, ...limits };
  for (const value of Object.values(cfg)) if (!Number.isSafeInteger(value) || value < 1) throw fail('ZIP_INVALID_LIMITS');
  if (typeof filePath !== 'string' || !filePath) throw fail('ZIP_PATH_REQUIRED');
  const before = await lstat(filePath);
  if (!before.isFile() || before.isSymbolicLink() || before.size < 22) throw fail('ZIP_INVALID_FILE');
  const handle = await open(filePath, 'r');
  const info = await handle.stat();
  if (info.dev !== before.dev || info.ino !== before.ino || info.size !== before.size) { await handle.close(); throw fail('ZIP_FILE_CHANGED'); }
  const readExact = async (buffer, position) => {
    let count = 0;
    while (count < buffer.length) {
      const result = await handle.read(buffer, count, buffer.length-count, position+count);
      if (!result.bytesRead) throw fail('ZIP_TRUNCATED');
      count += result.bytesRead;
    }
  };
  try {
    const tailLength = Math.min(info.size, 65535 + 22);
    const tail = Buffer.alloc(tailLength);
    await readExact(tail, info.size - tailLength);
    let pos = -1;
    for (let i = tailLength - 22; i >= 0; i--) {
      if (signature(tail, i, 0x06054b50) && i + 22 + u16(tail, i + 20) === tailLength) { pos = i; break; }
    }
    if (pos < 0) throw fail('ZIP_END_MISSING');
    if (u16(tail, pos + 4) || u16(tail, pos + 6) || u16(tail, pos + 8) !== u16(tail, pos + 10)) throw fail('ZIP_MULTIDISK_UNSUPPORTED');
    let count = u16(tail, pos + 10), dirSize = u32(tail, pos + 12), dirOffset = u32(tail, pos + 16);
    if (count === 65535 || dirSize === 0xffffffff || dirOffset === 0xffffffff) {
      const loc = pos - 20;
      if (!signature(tail, loc, 0x07064b50)) throw fail('ZIP64_LOCATOR_MISSING');
      if (u32(tail, loc + 4) !== 0 || u32(tail, loc + 16) !== 1) throw fail('ZIP_MULTIDISK_UNSUPPORTED');
      const eocd64Offset = safeU64(tail, loc + 8);
      const eocd64 = Buffer.alloc(56);
      await readExact(eocd64, eocd64Offset);
      if (!signature(eocd64, 0, 0x06064b50) || safeU64(eocd64, 4) < 44 ||
        u32(eocd64, 16) !== 0 || u32(eocd64, 20) !== 0 ||
        safeU64(eocd64, 24) !== safeU64(eocd64, 32)) throw fail('ZIP64_END_INVALID');
      count = safeU64(eocd64, 32);
      dirSize = safeU64(eocd64, 40);
      dirOffset = safeU64(eocd64, 48);
      if (dirOffset + dirSize > eocd64Offset) throw fail('ZIP64_END_INVALID');
    }
    if (count > cfg.maxEntries || dirSize > cfg.maxDirectoryBytes || dirOffset + dirSize > info.size - tailLength + pos) throw fail('ZIP_DIRECTORY_LIMIT');
    const dir = Buffer.alloc(dirSize);
    await readExact(dir, dirOffset);
    const entries = [];
    const names = new Set();
    let off = 0, totalBytes = 0;
    for (let index = 0; index < count; index++) {
      if (!signature(dir, off, 0x02014b50) || off + 46 > dir.length) throw fail('ZIP_CENTRAL_INVALID');
      const flags = u16(dir, off + 8), method = u16(dir, off + 10);
      const crc = u32(dir, off + 16);
      let compressedSize = u32(dir, off + 20), size = u32(dir, off + 24);
      const nameLen = u16(dir, off + 28), extraLen = u16(dir, off + 30), commentLen = u16(dir, off + 32);
      let localOffset = u32(dir, off + 42);const hostOS = dir[off + 5];
      const end = off + 46 + nameLen + extraLen + commentLen;
      if (end > dir.length) throw fail('ZIP_CENTRAL_INVALID');
      if (flags & 1 || flags & 0x40) throw fail('ZIP_ENCRYPTED_UNSUPPORTED');
      if (method !== 0 && method !== 8) throw fail('ZIP_COMPRESSION_UNSUPPORTED');
      if (size === 0xffffffff || compressedSize === 0xffffffff || localOffset === 0xffffffff) {
        let cursor = off + 46 + nameLen, found = false;
        const limit = cursor + extraLen;
        while (cursor < limit) {
          if (cursor + 4 > limit) throw fail('ZIP_EXTRA_INVALID');
          const code = u16(dir, cursor), length = u16(dir, cursor + 2), begin = cursor + 4;
          if (begin + length > limit) throw fail('ZIP_EXTRA_INVALID');
          if (code === 0x0001) {
            let p = begin;
            const take = () => { if (p + 8 > begin + length) throw fail('ZIP64_EXTRA_INVALID'); const n = safeU64(dir, p); p += 8; return n; };
            if (size === 0xffffffff) size = take();
            if (compressedSize === 0xffffffff) compressedSize = take();
            if (localOffset === 0xffffffff) localOffset = take();
            found = true;break;
          }
          cursor = begin + length;
        }
        if (!found) throw fail('ZIP64_EXTRA_MISSING');
      }
      const name = safeName(decodeName(dir.subarray(off + 46, off + 46 + nameLen)));
      const folded = name.toLowerCase();
      if (names.has(folded)) throw fail('ZIP_DUPLICATE_NAME');
      names.add(folded);
      const mode = hostOS === 3 ? (u32(dir, off + 38) >>> 16) : 0;
      if ((mode & 0xf000) === 0xa000) throw fail('ZIP_SYMLINK_UNSUPPORTED');
      if (size > cfg.maxEntryBytes || (size && size / Math.max(1, compressedSize) > cfg.maxExpansionRatio)) throw fail('ZIP_BOMB_LIMIT');
      totalBytes += size;
      if (totalBytes > cfg.maxTotalBytes) throw fail('ZIP_BOMB_LIMIT');
      if (localOffset >= dirOffset) throw fail('ZIP_OFFSET_INVALID');
      entries.push({ name, size, compressedSize, method, crc, flags, localOffset });
      off = end;
    }
    if (off !== dir.length) throw fail('ZIP_CENTRAL_TRAILING');
    async function entryStream(entry, {signal} = {}) {
      if (!entries.includes(entry)) throw fail('ZIP_ENTRY_UNKNOWN');
      if (signal?.aborted) throw fail('IMPORT_PAUSED');
      const local = Buffer.alloc(30);
      await readExact(local, entry.localOffset);
      if (!signature(local, 0, 0x04034b50) || u16(local, 8) !== entry.method || u16(local, 6) !== entry.flags) throw fail('ZIP_LOCAL_MISMATCH');
      const nameLength = u16(local, 26), extraLength = u16(local, 28);
      const filename = Buffer.alloc(nameLength);
      await readExact(filename, entry.localOffset + 30);
      if (decodeName(filename) !== entry.name) throw fail('ZIP_LOCAL_MISMATCH');
      const start = entry.localOffset + 30 + nameLength + extraLength;
      const end = start + entry.compressedSize;
      if (end > dirOffset || end > info.size) throw fail('ZIP_OFFSET_INVALID');
      if (!entry.compressedSize) {
        if (entry.size) throw fail('ZIP_INVALID_SIZE');
        return Readable.from([]);
      }
      const input = handle.createReadStream({ start, end: end - 1, autoClose: false, ...(signal ? {signal} : {}) });
      const inflate = entry.method === 8 ? createInflateRaw() : new PassThrough();
      let bytes = 0, runningCrc = 0xffffffff;
      const verify = new Transform({
        transform(chunk, _, done) {
          bytes += chunk.length;
          if (bytes > entry.size || bytes > cfg.maxEntryBytes) return done(fail('ZIP_BOMB_LIMIT'));
          runningCrc = crc32Update(runningCrc, chunk);
          done(null, chunk);
        },
        flush(done) {
          if (bytes !== entry.size) return done(fail('ZIP_SIZE_MISMATCH'));
          if (((runningCrc ^ 0xffffffff) >>> 0) !== entry.crc) return done(fail('ZIP_CRC_MISMATCH'));
          done();
        }
      });
      input.on('error', err => inflate.destroy(err));
      inflate.on('error', err => verify.destroy(err));
      input.pipe(inflate).pipe(verify);
      return verify;
    }
    async function fingerprint({signal} = {}) {
      const digest = createHash('sha256');
      let count = 0;
      for await (const chunk of handle.createReadStream({start:0,end:info.size-1,autoClose:false,...(signal ? {signal} : {})})) {
        count += chunk.length;
        digest.update(chunk);
      }
      if (count !== info.size) throw fail('ZIP_FILE_CHANGED');
      const after = await handle.stat();
      if (after.dev !== info.dev || after.ino !== info.ino || after.size !== info.size || after.mtimeMs !== info.mtimeMs || after.ctimeMs !== info.ctimeMs) throw fail('ZIP_FILE_CHANGED');
      return digest.digest('hex');
    }
    return Object.freeze({ entries, entryStream, fingerprint, size: info.size, directoryDigest: createHash('sha256').update(dir).digest('hex'), async close() { await handle.close(); } });
  } catch (err) { await handle.close(); throw err; }
}
export async function fingerprintFile(filePath) {
  const hash = createHash('sha256');
  for await (const chunk of createReadStream(filePath)) hash.update(chunk);
  return hash.digest('hex');
}
