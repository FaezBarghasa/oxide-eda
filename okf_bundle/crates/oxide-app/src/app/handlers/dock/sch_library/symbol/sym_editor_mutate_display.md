---
okf_version: "0.2"
type: Function
title: sym_editor_mutate_display
description: "Resolve the active `.snxsym` tab → its containing `.snxlib`,"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_display
language: rust
---

# sym_editor_mutate_display

Resolve the active `.snxsym` tab → its containing `.snxlib`,

## Signature

```rust
impl Oxide { pub(super) fn sym_editor_mutate_display(&mut self, mutator: F) -> bool }
```

## Type Parameters

- `F`

## Visibility

- `pub(super)`

## Docstring

Resolve the active `.snxsym` tab → its containing `.snxlib`,
run `mutator` on the library's display settings, then clear
the active editor's canvas cache so the change paints
immediately. Silently no-ops on lone-file edits or when
no Symbol editor is active.

## Source
Lines 18–41 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
