# Runtime acceptance matrix — evidence required

No live ZCode run was executed in the research session. All tests below remain UNRUN until performed in an authorized installed workspace.

## Identity and enablement
| ID | Test | Passing evidence |
|---|---|---|
| I01 | Project `.zcode/agents/zcode-workflow-specialist.md` loads | Agent listing names custom agent and source path |
| I02 | Subagent returns workflow design | Parent receives typed script and no child workflow run launched |
| I03 | `disabled` excludes dynamic workflow authoring | Tools absent and documented local-mode message |
| I04 | `onDemand` activates after explicit `/workflow` | Tools absent before, present after activation |
| I05 | `alwaysOn` exposes modern tools | Tool list includes CreateWorkflow and companions |
| I06 | Main skill gate blocks without loaded Skill | 428 before any confirmation; success after loading |
| I07 | Compaction requires proper skill restore/reload | Gate and loaded skill rechecked after compaction |

## Authoring, execution, controls
| ID | Test | Passing evidence |
|---|---|---|
| A01 | Working example typechecks | No diagnostics, displayed phase graph |
| A02 | Invalid type / missing phase blocked | Exact compiler diagnostics, no launch |
| A03 | Duplicate named agents blocked | Diagnostic or structured engine failure |
| A04 | Two independent actors execute under cap | Distinct child sessions and in-flight counts |
| A05 | Same actor serializes asks | FIFO journal order |
| A06 | Typed `.ask<T>` rejects malformed data | Schema validation failure visibly reported |
| A07 | `files.glob` and `git.status` read results journaled | Same resumed values and trace |
| A08 | `world.run` returns nonzero exit code normally | Script branches on exitCode and avoids false pass |
| A09 | report salvage on deliberate failure | Previously reported items visible in final notification |
| A10 | markdown artifact available in run | Artifact ID, version, content and primary card |
| A11 | User denies GUI confirmation | No run or mutation created |
| A12 | TUI/headless exception documented and controlled | Actual mode behavior recorded |
| A13 | Explicit provider choice honored | Identical model IDs and no silent provider switch |

## Durability and debugging
| ID | Test | Passing evidence |
|---|---|---|
| D01 | Background launch returns run ID | Run exists; not misreported completed |
| D02 | Read-only run introspection | List/Get show correct run/events |
| D03 | Cancel and resume | Same effective script, recovered completed nodes |
| D04 | Amend one changed ask | New run reuses matching named actor work; changed asks rerun |
| D05 | Retune only max concurrency live | Same run ID, observed gate behavior |
| D06 | Hole waits and FillWorkflowHole resumes branch | Correct hole ID and new graph phases |
| D07 | Escalation question visible to parent | Resolve tool resumes waiting child |
| D08 | Desktop debugger adapter connected | Graph and child-session links live; else FAIL/UNAVAILABLE |
| D09 | Saved workflow project/global shadowing | Correct resolved path and validated args |
| D10 | Parser/timeout/cap failures classified | Actual error code and journal state |
| D11 | Final report evidence vs UNRUN | Every claimed test supported by trace or command output |

## Evidence record template
```
TEST_ID:
STATUS: PASS | FAIL | BLOCKED | UNRUN
ZCODE_CLIENT_AND_VERSION:
WORKSPACE:
MODE:
RUN_ID:
EXPECTED:
OBSERVED:
COMMAND_OR_TOOL:
EXIT_CODE:
ARTIFACT_OR_LOG_PATH:
SOURCE_COMMIT:
NOTES:
```
Do not promote a test from UNRUN based only on upstream README claims or a hypothetical script.
