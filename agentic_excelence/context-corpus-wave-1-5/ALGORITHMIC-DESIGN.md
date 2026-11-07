# Corpus-driven agentic file compiler — algorithmic design v0.1
**Status:** source-informed design, no installed generator. **Target:** all 217 observed instances as *planning rows*, not 217 blindly regenerated instructions.

## 1. Why this is a compiler, not a template copier
Inputs are (a) pinned corpus source snapshots and exact source spans; (b) one target project/agent/task/runtime and its actual allowed tool surfaces; (c) source-of-truth graph and recent receipts; (d) owner policy and permissions; (e) provider-native file semantics. Output is a **proposed, inert, inspectable artifact** with an evidence manifest and unknowns. The requested agentic file must express verified task-specific information and must not smuggle a donor's instructions or self-promote archived material into owner policy.

There are three distinct questions:
1. Which *file kind* is warranted for this task and target harness?
2. Which *sections and claims* can be supported by currently valid evidence?
3. Should an authorized owner ever activate the resulting draft? (A separate step; never a side effect of generation.)

## 2. Source model and identities
Canonical source: `hq/registry/agentic-file-registry.v1.json` and `research/agentic-knowledge-graph/file-inventory.json` in Repos HQ at known SHA; matching record key `(repository, original path, Git blob SHA)`. The inventory has `file:001...file:217`; keep these IDs stable, do not reuse for newly generated target files. Derive target IDs from `(target repo, proposed path, provider/harness, source graph revision, approved policy version)`. Source heading variants are `section:<file>:<ordinal>` and have a semantic `section_type` from the 13-type catalog. Section headings are *observations*, never sufficient proof of a section's meaning.

A file instance is not a file type. `SKILL.md` can occur 52 times with distinct triggers and tools. `AGENTS.md` can be scoped at a root or nested location with different precedence. `SITREP.md` is a point-in-time evidence report, not a permanent policy. The 133 candidate naming patterns (121 unobserved in the census) are optional routing patterns, not a target file-creation quota. Eight donor snapshots are research-only.

## 3. Intermediate representation contracts (proposed)
### TargetFileRequest
- `targetRef`: repo/project, relative path, purpose and recipient agent/role.
- `harness`: OpenCode/Claude/Gemini/other native file convention with verified version.
- `taskRef` and current task; exact principal, policy, consent, environment, budget.
- `head`, `sourceGraphRevision`, `corpusVersion`, `policyVersion` and deterministic clock.
- `mode`: audit/plan/draft/validate; draft writes only to an inert LAB staging path.

### CandidateClaim
`{id, proposedText, role, sectionType, targetScope, sourceRefs:[{repo,path,sha,range}], evidenceClass, observationTime, trust, sensitivity, contradictions, status: supported|ambiguous|unsupported|historical}`.
Never derive authority from page rank. Any behavior that looks like a command is a quote/source datum until the current owner and runtime policy explicitly make it an instruction.

### FileDraftIR
`{schema, target, effectiveProviderSurface, canonicalOwners, sections:[{sectionType,title,claims,evidence,unknowns,tests}], excludedSources, sourceDigests, conflicts, estimatedTokens, activation:"inert", draftDigest}`.

## 4. Pipeline and algorithms
| ID | Algorithm | Deterministic responsibility | Required refusal or fallback |
|---|---|---|---|
| AF01 | Source pin + corpus census | Enumerate 217 rows, verify blob hashes, identify snapshots and lifecycle | Missing/mismatched source → mark unavailable, never fabricate |
| AF02 | Provider-native target resolver | Resolve AGENTS/CLAUDE/GEMINI/SKILL/config scope, precedence and inherited rules for the **actual** runtime | Unverified native loading → no activation |
| AF03 | File-type role classifier | Use `classifyPath`, `typesForTask` and `autoExpose` with 21 families, source authority and context trigger | Unknown type → review, not automatic new convention |
| AF04 | Section extraction + alignment | Parse frontmatter/JSON/Markdown; preserve heading/source ranges; map to 13 section types and unique variants | Same heading ≠ same meaning |
| AF05 | Evidence and claim builder | Pull exact source spans from HQ index/receipts, verified live graph paths and canonical task/status | Proposed/historical/claimed status cannot become runtime fact |
| AF06 | Context graph expander | Bounded typed BFS from task→owner→file→symbol→decision→test→skill; keep edge provenance | Missing typed edge → direct source fallback, mark unknown |
| AF07 | Policy + sensitivity gate | Check requester, destination, principal, source access, provider, task, secrets | Deny before ranking or caching; revocation wins |
| AF08 | Section requirement planner | Use family recipe; mark mandatory/conditional/forbidden sections, prerequisites and evidence obligations | Missing mandatory evidence → explicit UNKNOWN or blocked draft |
| AF09 | Claim conflict reconciler | Owner-approved/current > fresh observed receipts > actual code for implemented behavior > dated research; record contradictions | Never silently merge conflicting instructions |
| AF10 | Retrieval scorer | Relevance + graph proximity + role/trigger fit + evidence coverage + freshness + diversity − duplication/conflict/staleness | Avoid score inflation from 45 README or 52 SKILL instances |
| AF11 | Budgeted section allocator | Mandatory safety/owner rules first, then constrained maximum coverage under tokens/files; reserve exact user request and response | If hard contract cannot fit, fail rather than cut safety text |
| AF12 | Constrained draft synthesizer | Fill sections with sourced claims, literal verified commands, scoped examples, unknowns; optional model only for wording | Model text never invents tools, rights, status or citations |
| AF13 | Provider file serializer | Render Markdown/YAML/JSON and path-safe syntax for intended harness and target type | Reject invalid YAML/JSON, missing frontmatter or unsafe path |
| AF14 | Static and semantic validator | Schema, section obligations, provenance, hash, authority, citations, duplicate owners, leakage and test quality | Block on unsupported imperative claims or unpinned evidence |
| AF15 | Inert trial + adversarial verifier | Dry-run provider discovery and simulation; negative quoted-command, prompt injection, stale HEAD, policy denial | No live effect from a draft; independent verifier must see receipts |
| AF16 | Owner-gated promotion planner | Diff against exact target path, show inherited instructions, CAS expected HEAD, scoped approval | Never overwrite or activate automatically |
| AF17 | Continuous maintenance/diff | Compare live source graph with draft dependencies; invalidate affected sections and send draft back to review | Never silently change owner-approved active instructions |
| AF18 | Outcome evaluator | Compare failure rate, evidence recall, action correctness, retrieval cost, time to valid result | Retire or simplify if no measured improvement |

Reuse existing HQ `composeContext`, `verifyContext`, `filetypes`, `semantic-optimizers`, receipts and Rust HQKG read paths. The prior `algorithms.json` lists 10 experimental optimizers and 12 design candidates; map to them by ID, do not relabel them all as running Rust implementations.

### Candidate ranking (initial hypothesis, not tuned)
`score(c,t) = 4*taskCoverage + 3*verifiedEvidence + 2*roleMatch + 2*graphProximity + 1*recency + 1*sourceDiversity - 3*duplicateCost - 5*stalePenalty - 8*unresolvedConflict`.
All quantities normalized to [0,1]; denial/secret/historical restrictions are **hard filters**, not penalty terms. Optimize marginal evidence coverage rather than raw score totals. Add stable tie-break `sourceRef, range, digest`. Then run bounded knapsack with mandatory sections and hard token limits.

### Plan-to-draft pseudocode
```text
plan(request, registry, inventory, indexedSources, rustGraph, policy):
    assert pinned(request.head, registry.version, inventory.version)
    target = resolve_provider_surface(request.harness, request.targetRef)
    type = classify_target(target, registry)              # unknown -> review
    sections = family_recipe(type.family, type.role, target)
    candidates = collect_source_spans(indexedSources, rustGraph, request.taskRef)
    candidates = verify_source_hashes_and_acl(candidates, request.principal, policy)
    claims = reconcile_provenance_and_conflicts(candidates, current_owner_decisions)
    for section in sections:
        section.evidence = min_cost_cover(section.obligations, claims)
        if mandatory(section) and missing(section.evidence):
            section.status = "BLOCKED_MISSING_EVIDENCE"
    return plan_ir(sections, explicit_omissions, source_refs, activation="inert")

draft(plan):
    require all policy gates, valid source versions and supported target format
    render ONLY claims supported by the plan; append explicit UNKNOWNs
    validate frontmatter, syntax, citation coverage, nonduplication, scope, privacy
    output inert candidate + manifests + golden tests, not a live instruction file

activate(draft):
    separate OWNER/RUNTIME operation; NOT part of corpus or context engines
```

## 5. Sections: one shared ontology, many contextual variants
13 current shared section types: purpose, prerequisites, responsibilities, workflow, best practices, tools, context, safety, verification, recovery, lifecycle, examples, unique. The exact same heading can encode a different contract: a skill's "Verification" means executable positive/negative exercises; a SITREP's "Verification" means observed receipts; an AGENTS file's "Verification" is a required working rule; a schema's "Verification" is an invariant. Preserve file-specific headings and source sections rather than replacing them all with standardized boilerplate.

The 21 detailed family recipes, evidence inputs and all 217 proposed instance dispositions live in `CORPUS-217-DRAFT-PLANS.json`. The initial corpus records only eight families; 13 are *unobserved in this census* and may be drafted only when task demand and authority warrant them.

### The eight observed families — required synthesis differences
| Family | Corpus | Draft type and required differentiation |
|---|---:|---|
| 01 Agent entry/bootstrap | 53 | Current task/role/authority, exact workspace, safe first action, native instruction scope; no stale universal bootstrap ritual |
| 02 Provider-native | 7 | Real loader/version/path/precedence; vendor conventions never imply installation |
| 04 Context/retrieval | 46 | Source pointers, memory tiers, freshness, permissions; avoid lifting old personal memory from READMEs |
| 08 Tools | 5 | Current registered tool IDs, schemas, observed vs proposed capability, permission owner |
| 09 Skills | 67 | Specific activation trigger, reproducible steps, explicit stop/recovery, registered-discovery check; not 52 generic skills |
| 10 Config | 7 | Valid machine-readable schema, runtime version, default-off effects, rollback |
| 12 SITREP | 20 | Current pinned status with receipt distinctions; historical SITREP cannot install policy |
| 19 Handoff | 12 | Exact point-in-time scope, HEAD/digest, completed proofs, open risks, resume, one owner |

### Initial inspected exemplars
- `file:001` source `.opencode/agents/aion-chief.md` has OpenCode agent YAML frontmatter and explicit required-context/authority/reply sections. Do not generalize donor role-specific authority to VIVIM.
- `file:002` source `.opencode/skills/aion-skill-builder/SKILL.md` has name/description frontmatter and a skill-factory feedback loop; reuse its trigger/reuse/verify principles, not its AION-specific commands.
- `file:003` AION root `AGENTS.md` carries repository-wide normative instructions; target root AGENTS must be written from current target owner policy only, never cloned.
- `file:006` HANDOFF distinguishes synthetic fixture proof from missing live runtime proof. Preserve evidence classes and unresolved blockers.
- `file:010` SITREP proves that status is time-pinned and should be rechecked after concurrent publications.
- `file:097` tool registry distinguishes "reported"/"designed"/"unverified"/"connected"; a drafted tool catalog must revalidate live wiring.

## 6. Gold requirements and corpus retention
Each generated candidate must expose exact `sourceRef+hash+range`, family/section selection rationale, owner, provider target, effective precedence, sensitive-source omissions, contradiction list, token costs, proposed tests, and explicit UNKNOWNs. Store source snapshots as **inert .source.txt**; never make the recovered AGENTS/SKILL files active simply by moving them.

Do not optimize for 217 generated files. Optimize for the percentage of **warranted target-file slots** with verified useful drafts. Many observed source instances are duplicates of a *type*, archived handoffs, old state or donor content that should only influence research.

## 7. Architecture location and non-duplication
- Repos HQ: authority over taxonomy, filetype classification, source spans, current index/KB/decisions/canon and candidate drafting. Suggested additive seams: `hq/lib/agentic-draft-plan.mjs`, `hq/lib/agentic-draft-render.mjs`, `hq/test/agentic-draft*.test.mjs` after fresh audit.
- Rust HQKG: bounded graph neighborhood and typed traversal; no new corpus writer and no whole 94 MB DEV journal shipment to product.
- Web Harness: existing personal DCB and Host source permissions; only adapter consumption. Must never receive raw private corpus by default.
- Labs: goldset/simulation/reporting and the design manifest; no silent writes to canonical instructions.

## 8. Concrete acceptance for the first engineering slice
On pinned fixture roots, successfully generate at least one inert candidate each for `AGENTS.md`, `SKILL.md`, `HANDOFF.md`, `SITREP.md`, and `TOOLS.json` with reproducible digests; verify 217/217 metadata plans and 8 donor study-only classifications; reject stale hashes, fabricated commands, wrong provider, conflicting owners, injection in source text and insufficient mandatory evidence. Compare selected source coverage against static `composeContext` and never let an unapproved artifact appear in a live provider-native instruction path.
