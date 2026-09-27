---
okf_version: "0.2"
type: Function
title: context_submenu_msg_to_state
description: "Translate the pure-data [`SymbolContextSubmenuMsg`] into the"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/context_submenu_msg_to_state
language: rust
---

# context_submenu_msg_to_state

Translate the pure-data [`SymbolContextSubmenuMsg`] into the

## Signature

```rust
fn context_submenu_msg_to_state(
    msg: SymbolContextSubmenuMsg,
) -> crate::library::editor::symbol::state::SymbolContextSubmenu
```

## Docstring

Translate the pure-data [`SymbolContextSubmenuMsg`] into the
canvas/state-side [`crate::library::editor::symbol::state::SymbolContextSubmenu`].

## Source
Lines 481–488 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
