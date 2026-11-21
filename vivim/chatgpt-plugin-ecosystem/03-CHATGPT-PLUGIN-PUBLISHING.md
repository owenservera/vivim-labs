# Publishing a plugin in ChatGPT (November 2026)

**Scope:** distinguish an installable personal plugin, a private custom MCP connection, workspace publishing, public remote MCP submission and the restricted local desktop-only path. Official sources: https://developers.openai.com/plugins/build/plugins ; https://developers.openai.com/plugins/deploy/submission ; https://developers.openai.com/plugins/deploy/submission-errors ; https://developers.openai.com/plugins/deploy/desktop-only ; https://developers.openai.com/plugins/plugin-guidelines .

## The shortest answer: difficulty

| Goal | Engineering difficulty (our assessment, 1–5) | Main barrier |
|---|---:|---|
| Personal skills-only plugin | 1–2/5 | Manifest, instruction quality and local installation |
| Private custom remote MCP connection in ChatGPT | 2–3/5 | Working hosted MCP endpoint, auth and client connection |
| Share within eligible managed workspace | 2–3/5 | Workspace admin role and installation policies |
| Public skills-only plugin | 3/5 | Verified publisher, package validation, usefulness, safety scans, listing and review |
| Public **remote MCP** plugin | 4/5 | Reliable production HTTPS, OAuth, security, review evidence, automated tool scans and ongoing support |
| Public **desktop-only local MCP** plugin | 5/5 / access-gated | Requires approved-partner allowlisting; not simply a public ZIP upload |

Scores are subjective architectural complexity, not promised review timelines or guarantees of publication. A private plugin can work without public approval; a functioning private MCP endpoint alone is **not** a directory listing.

## What a plugin is in the 2026 system

The universal ChatGPT/Codex plugin directory supports:
- Skills: instructions and resources in packaged `skills/<skill>/SKILL.md`.
- MCP: one or more servers exposing tools with typed arguments/results, and optionally UI resources.
- Skills + MCP together for guided workflows.
- Some local Codex hooks; **public submission ZIPs containing lifecycle hooks or `apps`/`.app.json` references are currently ineligible**.

The modern portable package format is:
~~~text
my-plugin/
  plugin.json
  mcp.json                # if MCP-backed
  skills/
    example/SKILL.md      # if skills-backed
  assets/
    icon.png
    logo.png
~~~

Minimal identity (`plugin.json`):
~~~json
{
  "$schema": "https://agent-plugins.org/schemas/1.0.0/plugin.schema.json",
  "name": "vivim-companion",
  "version": "0.1.0",
  "description": "Approved VIVIM research and knowledge workflows."
}
~~~

Illustrative remote MCP configuration (`mcp.json`), **not a deployed server**:
~~~json
{
  "$schema": "https://agent-plugins.org/schemas/1.0.0/mcp.schema.json",
  "mcpServers": {
    "vivim": {
      "type": "streamable-http",
      "url": "https://mcp.example.com/mcp"
    }
  }
}
~~~

The endpoint is a **placeholder**. It cannot be submitted as a real endpoint. For a public remote plugin, supply a production HTTPS server and host/verify its domain. For OAuth, configure auth through `mcpServers.<name>.extensions["com.openai"].auth`, use supported client registration, and provide secrets only via the connection setup—not source code.

## Path A — private / local testing

1. Define a useful narrow capability (e.g. read a scoped wiki page or list approved tasks) and implement a corresponding MCP tool.
2. Test `tools/list` and `tools/call` with real non-sensitive fixtures, auth, structured errors and retry behavior.
3. Add the custom MCP endpoint in ChatGPT Plugins → Add custom MCP server (flow may depend on plan/workspace); acknowledge access warning and create as plugin.
4. For a skill-only plugin, create root `plugin.json` plus a `skills` directory and install from personal/repo marketplace where supported; optionally use Plugin Creator to scaffold.
5. Verify the actual ChatGPT tool picker and cross-platform behavior. Private personal installation is not a public listing.
6. When using a registered custom MCP connection, the technical ID can start with `plugin_asdk_app...` and can be passed to Plugin Creator for scaffold wiring.

## Path B — workspace publishing

Eligible workspace admins can publish a personal plugin to their workspace and assign roles under ChatGPT Plugins → Personal → plugin menu → Publish. This makes it visible to the workspace only; it is **not** the universal public directory. Workspace security policy may block self-service installation. The real admin/permission state must be inspected before planning rollout.

## Path C — publish to universal directory

1. Create a supported ZIP with root `plugin.json`, optional `skills/`, optional root `mcp.json`, correct icons and listing metadata. Do not include credentials.
2. Verify an individual or business publisher identity in the OpenAI Platform Dashboard; use an organization/project with appropriate apps management permissions.
3. For MCP: run a stable publicly reachable HTTPS endpoint, register OAuth callback and reviewer credentials when applicable; connect it in portal; serve the exact domain challenge token at `/.well-known/openai-apps-challenge`.
4. Upload ZIP; resolve package-format errors and skill/security scan findings.
5. Scan remote tools; explicitly annotate and **justify** `readOnlyHint`, `openWorldHint` and `destructiveHint` on **every tool**. Ensure external effects are honest, audited and bounded.
6. Provide website, support, privacy and terms URLs; icon; complete metadata, demo video; exactly **five positive** and **three negative** test cases, release notes and reviewer-ready access for OAuth-backed tools.
7. Submit for review, address findings; after approval choose when to publish.
8. Maintain service health and compliance; the platform can rescan hosted tools daily and pick up eligible server-side changes automatically. ZIP metadata/skills updates require a new package upload.

**Published package format and final listing validation are different.** Submission errors can occur after a ZIP passes basic validation.

## Public directory metadata constraints (current source)

OpenAI's submission-errors reference currently lists:
- Package name <=64 ASCII alphanumeric/`_`/`-`; semantic version <=64 characters.
- Display name <=30 characters; short description <=30; long description <=4000.
- Developer name <=80; up to 20 capabilities; up to 3 distinct starter prompts.
- Remote MCP requires HTTPS website, privacy, terms and support URLs (<=1024 chars each), plus main-use-case demo recording.
- Skill scans must pass; for remote MCP a successful current tool scan/domain verification are mandatory.
- MCP tools require accurate explicit behavioral annotations **with justification**.
- OAuth submissions need usable demo credentials; reviewers must not be blocked by non-reviewable 2FA.

Reconfirm exact constraints on the day of submission; platform requirements evolve.

## Path D — desktop-only plugins: genuinely different

OpenAI describes public **local stdio MCP** submissions as limited to approved partners. A partner must arrange organization allowlisting with an OpenAI representative. These are marked Desktop only and run only in eligible local ChatGPT desktop threads. A normal account that can install a private local plugin does not automatically obtain public local-MCP publisher access.

**Strategic consequence:** VIVIM should publish a remote HTTPS MCP facade first if public distribution is important, while running local device capability inside its owner-controlled Host and pairing flow. Do not plan product launch around obtaining desktop-only allowlisting.

## Content and review policy traps for VIVIM

- Source clearly says primarily **unofficial third-party connectors / passthrough relays** cannot be approved. VIVIM's signed-in browser provider automation should not be packaged as a public unofficial ChatGPT/Claude connector. Discuss scope and rights before public submission; be transparent about any third-party integration.
- Hidden writes, broadly privileged arbitrary shell calls, user impersonation, exfiltration, undocumented data collection and attempts to bypass review will make approval harder and may breach policy.
- A general-purpose filesystem-and-shell MCP with unrestricted access poses a higher trust and review burden than an explicit typed, user-consented product API.
- Keep Host approval as the **real** effect gate even if ChatGPT prompts for confirmation; model/interface approvals cannot replace application policy and receipts.
- Strong UI can be optional. A clean, structured text-and-data MCP plugin is a legitimate first release; avoid premature iframe/widget work.

## Recommended sequence

**Era 1 — Private usefulness:** One read-only `wiki.search` and `task.list` tool; personal install, no published listing.  
**Era 2 — Production remote MCP:** Rust HTTPS facade, OAuth, device/world scoping, audited Host-effect proposals, recovery and revoke; exercise negative cases.  
**Era 3 — Public publishing:** Verified VIVIM business identity, copy/legal/support readiness, domain verification, reviewer demo, complete annotation suite and staged submission. Do not pursue desktop-only partner path until there is a clear need.
