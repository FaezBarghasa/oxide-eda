---
okf_version: "0.2"
type: Function
title: sim_set_pin_node_whitespace_removes_key
description: Whitespace-only values trim to empty and remove the key —
resource: crates/oxide-app/src/library/editor/sim/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:12:05Z"
concept_id: crates/oxide-app/src/library/editor/sim/mod/sim_set_pin_node_whitespace_removes_key
language: rust
---

# sim_set_pin_node_whitespace_removes_key

Whitespace-only values trim to empty and remove the key —

## Signature

```rust
fn sim_set_pin_node_whitespace_removes_key()
```

## Decorators

- `test`

## Docstring

Whitespace-only values trim to empty and remove the key —
mirrors the trimming that the SaveDraft path does on text input.
[test]

## Source
Lines 448–469 in `crates/oxide-app/src/library/editor/sim/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/editor/sim/mod.md) |
| calls | [fixture_editor](/crates/oxide-app/src/library/editor/sim/mod/fixture_editor.md) |
| calls | [apply](/crates/oxide-app/src/library/editor/sim/mod/apply.md) |
