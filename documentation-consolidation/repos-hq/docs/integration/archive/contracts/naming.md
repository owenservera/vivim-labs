# Frozen contract — naming across three repositories

**Schema:** `hq.naming/1` · **Status:** frozen for waves 0–3 · **Owner:** L0
**Gate:** F0 · **Blocks:** every lane, every schema, every doc
**Evidence:** GAP-016

---

## 1. The collision

Two different things are called **World**:

| Repo | Meaning | Evidence |
| --- | --- | --- |
| harness / kernel | the user's **personal knowledge world**: records with privacy class, provenance, tombstones | `vivim-web-harness/docs/design/APK07-mobile-runtime-world-offline.md:51` (`world_projection`) |
| TierFlow / vivim-ui | a **UI document**: objects, layers, shapes, data flows on an infinite 3D canvas | `vivim-ui/docs/WORLD-MODEL-SPEC.md:17` |

Both meanings are legitimate. The **word** is not. A lane that reads one meaning and writes the
other produces a schema that is wrong in a way no test will catch, because both are "the World".

## 2. Frozen vocabulary

| Term | Means | Never means |
| --- | --- | --- |
| **Personal World** | the user's knowledge base, owned by the VIVIM Host | a UI layout |
| **`world_projection`** | the phone-visible read model of the Personal World | a published UI revision |
| **surface document** | the declarative UI layout: objects, layers, shapes, relationships, data flows | personal knowledge |
| **surface revision** | one immutable published version of a surface document | a Personal World revision |
| **draft** | private working state bound to a surface revision | a Personal World draft |
| **publish / rollback** | CAS-advancing a surface revision | writing knowledge |
| **command id** | client-generated idempotency key | a provider request id |
| **epoch / seq** | process-scoped resume cursor | a schema version |
| **capability** | what a credential may attempt | an approval |
| **grant** | the owner's recorded permission for a device or actor | an execution |

## 3. Legacy names that stay, and why

| Legacy name | Where | Rule |
| --- | --- | --- |
| `worldId` | TierFlow wire + routes (`/api/ui/worlds/:worldId`) | **Kept for protocol compatibility.** Every schema, doc and comment that uses it must say "legacy name for a surface document". Removing it is a protocol version bump, not a rename |
| `WORLD-MODEL-SPEC.md` | vivim-ui docs | Kept as a filename; its first line must state that it describes a surface document |
| `World` in harness code | `apps/vivim-host`, `docs/design` | Kept; it is the Personal World and is unambiguous inside that repo |

## 4. Forbidden aliases in new material

These are lint findings, not style preferences:

| Forbidden in new text | Replace with |
| --- | --- |
| "UI world" | surface document |
| "world document" (for a UI document) | surface document |
| "screen" (for the declarative surface) | surface |
| "theme" (for the whole surface document) | surface document; "theme" is only the colour/typography sub-object |
| "the World" without a qualifier | Personal World or surface document |
| "sync the world" | sync the Personal World, or publish the surface |

## 5. Enforcement

1. New documents and schemas must use the frozen vocabulary. A reviewer rejects on sight.
2. `scripts/integration/check.mjs` may be extended with an alias scan over newly added markdown;
   until then this is a review rule, stated as such rather than pretended to be automated.
3. Any schema that carries `worldId` must carry the comment `legacy name for surface document`
   within two lines of the field.
4. A rename proposal is a version bump with a migration note; it is never an in-place edit while
   lanes are live (§4 of the roadmap).

## 6. Why freeze this before code

The recorded failure in `vivim-ui/docs/SESSIONS-NEXT-STEPS-2026-11-03.md:94-99` was a contract
rewritten under four concurrent lanes. Naming is the cheapest contract to freeze and the most
expensive one to discover late: it changes schemas, routes, database columns and every design
conversation at once.

## 7. Acceptance

1. `docs/integration/contracts/naming.md` exists and is referenced by every lane's
   `frozenContracts`.
2. No new file added by a wave-1+ lane uses a forbidden alias.
3. Any `worldId` field added after this contract carries the legacy comment.
4. The roadmap §2.1 and this document state the same rule, word for word on the term names.
