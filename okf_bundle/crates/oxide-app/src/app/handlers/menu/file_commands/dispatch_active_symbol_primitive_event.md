---
okf_version: "0.2"
type: Function
title: dispatch_active_symbol_primitive_event
description: "Resolve the active tab; if it's a `.snxsym` standalone editor"
resource: crates/oxide-app/src/app/handlers/menu/file_commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/file_commands/dispatch_active_symbol_primitive_event
language: rust
---

# dispatch_active_symbol_primitive_event

Resolve the active tab; if it's a `.snxsym` standalone editor

## Signature

```rust
impl Oxide { fn dispatch_active_symbol_primitive_event(
        &mut self,
        msg: crate::library::messages::SymbolEditorMsg,
    ) -> Option<Task<Message>> }
```

## Docstring

Resolve the active tab; if it's a `.snxsym` standalone editor
fire `msg` against its `path`. Returns `None` when no Symbol
editor is active so the menu item silently no-ops on other
tab kinds (mirrors `MenuMessage::Save`-style guards).

## Source
Lines 141–159 in `crates/oxide-app/src/app/handlers/menu/file_commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [file_commands](/crates/oxide-app/src/app/handlers/menu/file_commands.md) |
