# PC Host vs Android — initial hypotheses

Legend: E = likely feasible via browser extension (must test); N = requires native Android bridge; H = remains Host-mediated under DEC-MOBILE-001; X = impossible through extension alone.

| Function | Initial path | Key limitation / test |
| --- | --- | --- |
| Observe DOM on signed-in website | E | Permissions, site restrictions, frame handling |
| Read page text/metadata, explicit capture | E + N | Scope and owner consent |
| Fill web forms, click permitted controls | E | Site behavior, mobile DOM, dynamic rendering |
| Tabs and navigation | E | Browser API coverage, foreground policy |
| Multi-provider tab orchestration | E/H | Background lifecycle and mobile UI |
| Script injection | E | API compatibility and isolation |
| Reliable background automation | N/H | Android process lifecycle and browser suspension |
| Desktop CDP / remote-debugging equivalence | X/H | Extension APIs are not unrestricted desktop CDP |
| Access arbitrary other Android apps | X/N | Android app sandbox, consent-bound native integration |
| Share sheet, microphone, device files | N | Android runtime permissions and intents |
| Full filesystem / OS process control | X/N/H | Sandboxing and Android restrictions |
| Persistent offline raw evidence capture | N | Allowed by DEC-MOBILE-001 as device-local capture only |
| Canonical World / KB mutation | H | Host is sole authoritative writer |
| Provider effect execution / send | H | Host is sole provider/effect send path |
| Owner approvals and receipts | N + H | Phone may approve, Host validates/journals |
| Host-offline browser interaction | E (experimental) | No silent queued provider effects or KB promotion |

A mobile-native Host replacement is **not** established by browser extension support. Distinguish browser automation feasibility from authorized VIVIM architecture.
