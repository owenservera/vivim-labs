# Paired PWA reprogrammability experience

## 1 — select
User selects one or more cards, labels, buttons or surfaces by touch, keyboard, natural language or Studio selection.
Trusted Host/PWA model resolves current component IDs; ambiguous names must ask clarification.

## 2 — discover
The property inspector groups central engines into Appearance, Layout, Text, Behavior, Data, Accessibility and Advanced.
For each engine, expose semantic applicability separately from source/renderer availability and current permission.
A slider can declare Color, Border and Focus options without implementing any of those engines.

## 3 — change
User: 'Make these cards dark blue with a 12px corner radius and a soft shadow.'
VEC selects the Color, Corners and Shadow engines, calls typed validators, resolves tokens, plans dependencies,
and emits one atomic, inert, revision-bound UI config change. No Rust compilation or provider reasoning for known intent.

## 4 — preview
Explain all edits, affected components and unavailable fields; preserve reduced motion, a11y and private-data scopes.
No arbitrary CSS, JavaScript, callbacks, unknown component props or speculative Host grant.

## 5 — activate
Owner explicitly approves. Existing Host checks actor/device/current UI revision and current grant.
Existing single publisher commits; both PWA and Windows receive versioned projected view and receipt.

## 6 — undo and reuse
Undo restores UI revision (not irreversible external effects). Save named tokens and reusable appearance recipes
under existing personal program state, with redacted share/export option and versioned dependencies.

## First accepted target
Paired PWA: one visibly renderable, deterministic UI configuration operation with tests, Host CAS, receipts and Undo.
Current package only generates pure data candidates; renderer adoption and physical pairing tests remain pending.
