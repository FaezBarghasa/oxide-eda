---
okf_version: "0.2"
type: Function
title: blocking_modal_outranks_a_quit_gate_set_behind_it
description: "#514 round 4 — refutes a round-3 claim. `app_quit_confirm_open`"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/blocking_modal_outranks_a_quit_gate_set_behind_it
language: rust
---

# blocking_modal_outranks_a_quit_gate_set_behind_it

#514 round 4 — refutes a round-3 claim. `app_quit_confirm_open`

## Signature

```rust
fn blocking_modal_outranks_a_quit_gate_set_behind_it()
```

## Decorators

- `test`

## Docstring

#514 round 4 — refutes a round-3 claim. `app_quit_confirm_open`
and a `has_blocking_modal` member CAN both be `true` at once: Alt+F4
/ native window close sets `app_quit_confirm` unconditionally
(`handle_app_quit_requested`, no `has_blocking_modal` check on that
path anywhere) — so an export failure followed by Alt+F4 leaves
both set while only the blocking modal's card paints. The guard at
the top of `escape_message` is what keeps Esc resolving to the
visible card in that reachable state, not the invisible quit gate;
this test exercises all four members of the group against it.
[test]

## Source
Lines 1105–1142 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
