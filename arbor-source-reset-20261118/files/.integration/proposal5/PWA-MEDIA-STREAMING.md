# ARBOR PWA media streaming — configured local files

Direct Host PWA uses the existing HQ library.json and favorites.json as sole title authority. A complete download is NOT automatically a playable file. The owner supplies an explicit file mapping.

Set VIVIM_MEDIA_STORE_DIR to the existing HQ torrent-store directory (on Windows the default is C:/VIVIM/channel/vivim/torrent-store). Set VIVIM_MEDIA_FILES_ROOT to an absolute, owner-controlled root directory containing playable files.

Create an optional playback.json in the same torrent-store directory, with exactly this JSON shape:

```json
{ "version": 1, "entries": [ { "libraryId": "exact-existing-HQ-library-row-id", "relativePath": "Movies/Example.mp4" } ] }
```

Only a complete HQ row with a valid mapping and an actual non-symlinked file is playable. The path is relative to VIVIM_MEDIA_FILES_ROOT. Supported file formats: mp4, m4v, webm, mp3, m4a, ogg. Device/browser codec support varies. No arbitrary directory scan, URL, magnet or torrent source path is allowed.

The owner-approved Direct device calls POST /mobile/api/media/open with its control:read grant, exact Origin and explicit x-vivim-media header. The Host returns a same-origin stream URL and an HttpOnly Secure SameSite=Strict cookie. The ticket stays in process memory and binds the file fingerprint, session, media ID and grant. Revoke the device to stop subsequent reads. GET/HEAD support full body, one byte range 206, and unsatisfiable 416. Node streams handle backpressure and abort. The service worker does not cache media or authenticated APIs.

Device pairing, launch, the actual browser media codecs and Windows acceptance are UNRUN until confirmed by executable tests and a paired Android device. The PWA player is usable only over authenticated Tailscale Direct, not Cloud or loopback desktop preview. Native buffering exists, but durable offline media cache and transcoding are NOT implemented.

## Catalog pagination

The Direct mobile catalog uses read-only `GET /mobile/api/media/library?offset=N`, with a bounded integer offset (0 to 100000), a 25-item page size and an exact total. Previous/Next on World re-read metadata only. `POST /mobile/api/media/open` still resolves an opaque ID against the full HQ catalog, so a legitimately mapped movie beyond page one is playable. Cloud and desktop preview never inherit a phone stream grant.

## Revocation during playback

The active stream is rechecked against the same Host grant/session at initial read, after at most 256 KiB of unchecked bytes (or 200 ms of elapsed streaming), and on stream completion. Revoking a device terminates a long-running stream at the next validation boundary, not just on the next HTTP range request. This uses the existing enrollment authority and Node backpressure stream, not a new token store or authorizer. Synthetic Node tests are committed; Windows and Android runtime checks remain UNRUN.
