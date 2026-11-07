# Extension family: Spanish Legal Practice

Family ID: `es-legal-practice`. **Lab-only exploratory scaffold.** Initial testbed for one family composed across many canonical VIVIM extension points, not a separate legal AI product or a fork of VIVIM.

## What it should enable

An owner-controlled workspace supporting Spanish legal professionals with research, citations, matter organization, document review, drafting, calendar/deadline candidates, and policy-aware automated workflows. Specialties can later have separate packs (civil, mercantile, laboral, administrativo, contencioso, penal, fiscal, inmobiliario) sharing the common platform; actual coverage and accuracy are unverified.

## Suggested module map

| Module ID | Experimental role | Candidate attachment points |
|---|---|---|
| es-legal-knowledge | Spanish legal-source discovery, tagging, citation provenance, version-aware retrieval | EP-02, EP-03, EP-04, EP-17 |
| es-legal-language | Spanish legal terminology, multilingual intent mappings, structured matter vocabulary | EP-06, EP-07, EP-16 |
| es-legal-agents | Research, issue spotting, redlining/drafting, conflict and completeness checks | EP-05, EP-08, EP-09 |
| es-legal-workspace | Matter/project UI, legal citation cards, document templates, deadlines | EP-11, EP-12, EP-13, EP-16 |
| es-legal-governance | Confidentiality, role isolation, client consent, risk checks, evaluated outcomes | EP-14, EP-15, EP-19 |
| es-legal-compute | Optional specialized Rust indexing/ranking/algorithm engine, only if proven useful | EP-01, EP-20 |

A module should use *multiple* extension points only when needed; avoid duplicating general-purpose capabilities inside a domain pack. Skills, data packs and declarative templates should come before native code where possible.

## Profile experiments

- **Foundation** — glossary, citation/document templates, legal-source reference workflow, manual review.
- **Practice** — matter taxonomy, structured drafting, evidence-backed retrieval, constrained deadline candidates.
- **Advanced analysis** — cross-document comparison, temporal legislation checks, multi-step research and provenance review.
- **Organization-custom** — approved firm templates, partitioned client workflows, tailored policies and team roles.

Profiles express feature composition, not confirmed shipping tiers, performance guarantees or permission grants.

## Spanish jurisdiction and source questions

Validate legal-source freshness and reliability individually: BOE for legislation (including consolidated text and amendment dates), CENDOJ/CGPJ for judgments where applicable, and official territorial/administrative sources as needed. Verify access rights and allowed reuse before downloading or indexing; official publication does not automatically grant unrestricted repackaging. Distinguish EU, state, autonomous-community and local scope, and effective date vs publication date. Track cited articles, legislation versions, court/case identifiers and source URL. Do not assume court filings, representation, calendaring calculations or legal advice can be automated safely.

Data-classification defaults to confidential; client/matter separation, access restrictions, legal privilege, retention, GDPR considerations and auditability are first-class. Generated answers are drafts for professional review, not verified legal opinions.

## Initial lab scenarios

1. Research question -> official source lookup -> provenance-preserving summary.
2. Draft contract clause -> explicit legal basis -> citations and uncertainty visible.
3. Upload a scanned or malformed contract -> restricted parser -> structure/citation validation.
4. Compare two versions of a legal document without cross-matter leakage.
5. Ask in English and Spanish for same typed research action; confirm equivalent IR.
6. Request deadline assistance -> identify missing forum/procedure/timezone/calendar facts; never invent certainty.
7. Propose outbound client communication -> Host owner approval -> verified or unknown receipt.
8. Attempt cross-matter exfiltration and prompt injection in source documents -> deny and record.
9. Switch profile, revoke device, uninstall extension -> retain governed originals, purge derived unauthorized projections.
10. Load a failing Rust/domain analysis node -> isolate fault, preserve canonical engine and audit logs.

## Next validation pass

For every module, inspect actual Repos HQ/TierFlow/Harness source, create an evidence-linked mapping to specific current interfaces, note whether an extension hook *exists*, and select a thin prototype that runs only inside Labs. Do not add a new provider API route, browser controller, command registry, knowledge authority, UI publisher or Android app.
