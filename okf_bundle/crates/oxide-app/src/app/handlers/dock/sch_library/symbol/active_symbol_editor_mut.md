---
okf_version: "0.2"
type: Function
title: active_symbol_editor_mut
description: "Borrow-mut the active tab's `SymbolEditorState`, if the"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/active_symbol_editor_mut
language: rust
---

# active_symbol_editor_mut

Borrow-mut the active tab's `SymbolEditorState`, if the

## Signature

```rust
impl Oxide { fn active_symbol_editor_mut(&mut self) -> Option<&mut crate::app::SymbolEditorState> }
```

## Docstring

Borrow-mut the active tab's `SymbolEditorState`, if the
active tab is a Symbol editor. Returns `None` for any other
tab kind so the SCH Library handlers can exit fast.

## Source
Lines 590–600 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
