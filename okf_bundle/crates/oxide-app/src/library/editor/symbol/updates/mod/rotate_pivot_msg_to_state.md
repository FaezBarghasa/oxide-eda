---
okf_version: "0.2"
type: Function
title: rotate_pivot_msg_to_state
description: Translate pure-data rotate pivot messages into Symbol-state pivot mode.
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/rotate_pivot_msg_to_state
language: rust
---

# rotate_pivot_msg_to_state

Translate pure-data rotate pivot messages into Symbol-state pivot mode.

## Signature

```rust
fn rotate_pivot_msg_to_state(
    msg: SymbolRotatePivotMsg,
) -> crate::library::editor::symbol::state::GraphicRotationPivotMode
```

## Docstring

Translate pure-data rotate pivot messages into Symbol-state pivot mode.

## Source
Lines 591–599 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| called_by | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
