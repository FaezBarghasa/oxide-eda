---
okf_version: "0.2"
type: Function
title: context_target_msg_to_state
description: "Translate the pure-data [`SymbolContextTargetMsg`] into the"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/context_target_msg_to_state
language: rust
---

# context_target_msg_to_state

Translate the pure-data [`SymbolContextTargetMsg`] into the

## Signature

```rust
fn context_target_msg_to_state(
    msg: SymbolContextTargetMsg,
) -> crate::library::editor::symbol::state::SymbolContextTarget
```

## Docstring

Translate the pure-data [`SymbolContextTargetMsg`] into the
canvas/state-side [`crate::library::editor::symbol::state::SymbolContextTarget`].

## Source
Lines 468–477 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| called_by | [apply_symbol_context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu.md) |
