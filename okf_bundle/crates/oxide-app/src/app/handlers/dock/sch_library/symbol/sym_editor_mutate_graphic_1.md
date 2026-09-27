---
okf_version: "0.2"
type: Function
title: sym_editor_mutate_graphic
description: "Helper — apply a closure to the graphic at `idx` on the active"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_graphic_1
language: rust
---

# sym_editor_mutate_graphic

Helper — apply a closure to the graphic at `idx` on the active

## Signature

```rust
pub(super) fn sym_editor_mutate_graphic(&mut self, idx: usize, mutator: F) -> bool
```

## Type Parameters

- `F`

## Visibility

- `pub(super)`

## Docstring

Helper — apply a closure to the graphic at `idx` on the active
Symbol editor. Sibling of [`sym_editor_mutate_pin`] for
per-shape Properties edits. Silently returns when no Symbol
editor is active or the index is out of range.

## Source
Lines 90–106 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
