# User journey acceptance matrix — R001
All rows are PROPOSED / UNVERIFIED in this document. IDs local to product journey scope, not replacements for historical J001–J120. Engineering references point to ARBOR issue #33 and Proposal 5 B01–B12. Existing contracts remain authoritative.

| Journey | User story and observable result | Failure/recovery | Existing references |
| --- | --- | --- | --- |
| V1-J01 First contact | Fresh Windows install opens native GPUI quiet composer; choose workspace outside repo; restart preserves it | Invalid path/low disk explained; no private data in repo; no donor dependency | B01/B10/B12; S02–S05/S13 |
| V1-J02 Phone continuity | Pair actual Android PWA to PC, type on each, observe same live composer/state on both, revoke and re-pair | Offline indication; preserve draft; stale device cannot invoke action | B04/B09; S06–S11/S14 |
| V1-J03 My AI | Sign into own ChatGPT, start/select thread, send exact approved text once; view genuine response; resume same thread from phone/PC | Account mismatch/login/error clear; uncertain send never blindly replayed | B02/B03; A01–A12/A16 |
| V1-J04 Attachment | Select permitted workspace file, preview file/destination, submit via supported provider path, see attachment/result | Unsupported size/type/missing file explicit; revoke/deny no send | C03; B03/B06 |
| V1-J05 My machine | Type open [registered app], search [workspace], inspect available command; show candidate before effects; perform supported local action with receipt | Clarify ambiguous target, show unsupported intent, prevent stale effect | B05/B06; U06–U09/U11 |
| V1-J06 My knowledge | As user types and receives actual AI response, live Wiki shows grounded source-linked entries; open and correct a user-authored note; restart and read revision | Stale/empty/unavailable states, explicit corrections, scoped access | B07/B08; U05/O01/O03–O05 |
| V1-J07 My workspace | Browse user-selected workspace, inspect available local capabilities and sources, search and open permitted file | Nonexistent, denied, offline and unsupported states truthful | B07/B11; U11 |
| V1-J08 Continuity | Lose connectivity, restart app/Host, reconnect phone, recover conversation and journal, inspect receipt and stop/revoke access | No duplicated external effect; no silent data loss; rollback verified | B06/B10/B12; A16/S13/R06 |

First end-to-end script: install -> workspace -> optional account connection -> pair -> new ChatGPT conversation -> send Harbor/local-first/JSON export, rename Cedar and add offline mode, ask for current name and three requirements (historic owner pilot D14) -> attach an approved harmless file -> search workspace -> live Wiki with source refs -> restart/revoke/reconnect. Use real product, physical phone and explicitly authorized provider interaction. No synthetic receipt. Proposed order can be revised after interview. The user may skip account onboarding to use supported local features.
