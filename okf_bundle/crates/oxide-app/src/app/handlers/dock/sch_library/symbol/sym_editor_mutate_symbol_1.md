---
okf_version: "0.2"
type: Function
title: sym_editor_mutate_symbol
description: "Helper — apply a closure to the active symbol (`Symbol`) on"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_symbol_1
language: rust
---

# sym_editor_mutate_symbol

Helper — apply a closure to the active symbol (`Symbol`) on

## Signature

```rust
pub(super) fn sym_editor_mutate_symbol(&mut self, mutator: F) -> bool
```

## Type Parameters

- `F`

## Visibility

- `pub(super)`

## Docstring

Helper — apply a closure to the active symbol (`Symbol`) on
the active Symbol editor. Used by Properties Component
section edits (designator / comment / description / type /
mirrored). Runs the standard dirty/refresh cycle. No-op when
no Symbol editor is the active tab.

## Source
Lines 71–84 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
