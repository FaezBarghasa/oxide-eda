---
okf_version: "0.2"
type: Function
title: quit_gate_outranks_a_dialog_layered_under_it
description: The app-quit gate can be triggered while a shallower dialog (e.g.
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/quit_gate_outranks_a_dialog_layered_under_it
language: rust
---

# quit_gate_outranks_a_dialog_layered_under_it

The app-quit gate can be triggered while a shallower dialog (e.g.

## Signature

```rust
fn quit_gate_outranks_a_dialog_layered_under_it()
```

## Decorators

- `test`

## Docstring

The app-quit gate can be triggered while a shallower dialog (e.g.
Preferences) is still open; the gate the user just triggered must
win, not the dialog underneath it.
[test]

## Source
Lines 1081–1093 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
