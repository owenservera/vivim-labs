// Incremental strict UTF-8 parser of a top-level JSON array of objects.
// Bounded object memory and no dependency on a specific ChatGPT export layout.
const fail = code => Object.assign(new Error(code), { code });
export async function* parseJsonObjectArray(stream, {maxObjects=2_000_000,maxObjectBytes=64*1024*1024}={}) {
  if (!Number.isSafeInteger(maxObjects) || maxObjects < 1 || !Number.isSafeInteger(maxObjectBytes) || maxObjectBytes < 128) throw fail('JSON_BOUNDS');
  const decoder = new TextDecoder('utf-8', {fatal:true});
  let state='open', nesting=0, quoted=false, escape=false, pieces=[], length=0, count=0;
  const append = fragment => { if (!fragment) return; pieces.push(fragment); length += Buffer.byteLength(fragment); if (length > maxObjectBytes) throw fail('JSON_OBJECT_LIMIT'); };
  const chunks = (async function* () {for await(const b of stream)yield decoder.decode(b,{stream:true}); yield decoder.decode();})();
  for await (const text of chunks) {
    let segmentStart=-1;
    for (let i=0;i<text.length;i++) {
      const ch=text[i];
      if(state==='open') {if(/\s/.test(ch))continue;if(ch!=='[')throw fail('JSON_TOPLEVEL_ARRAY_REQUIRED');state='value';continue;}
      if(state==='done'){if(!/\s/.test(ch))throw fail('JSON_TRAILING_DATA');continue;}
      if(state==='value') {
        if(/\s/.test(ch))continue;
        if(ch===']'){state='done';continue;}
        if(ch!=='{')throw fail('JSON_OBJECT_REQUIRED');
        state='object';nesting=1;quoted=false;escape=false;segmentStart=i;continue;
      }
      if(state==='comma') {
        if(/\s/.test(ch))continue;
        if(ch===','){state='next';continue;}
        if(ch===']'){state='done';continue;}
        throw fail('JSON_SEPARATOR_REQUIRED');
      }
      if(state==='next'){
        if(/\s/.test(ch))continue;
        if(ch!=='{')throw fail('JSON_OBJECT_REQUIRED');
        state='object';nesting=1;quoted=false;escape=false;segmentStart=i;continue;
      }
      if(state==='object') {
        if(quoted){if(escape)escape=false;else if(ch==='\\')escape=true;else if(ch==='"')quoted=false;}
        else if(ch==='"')quoted=true;
        else if(ch==='{'||ch==='[')nesting++;
        else if(ch==='}'||ch===']')nesting--;
        if(nesting<0)throw fail('JSON_BRACKET_MISMATCH');
        if(nesting===0){
          append(text.slice(segmentStart<0?0:segmentStart,i+1));segmentStart=-1;
          const raw=pieces.join('');pieces=[];length=0;count++;
          if(count>maxObjects)throw fail('JSON_OBJECT_COUNT_LIMIT');
          let object;try{object=JSON.parse(raw);}catch{throw fail('JSON_INVALID_OBJECT');}
          if(!object||typeof object!=='object'||Array.isArray(object))throw fail('JSON_OBJECT_REQUIRED');
          state='comma';yield object;
        }
      }
    }
    if(state==='object')append(text.slice(segmentStart<0?0:segmentStart));
  }
  if(state!=='done')throw fail('JSON_TRUNCATED');
}
