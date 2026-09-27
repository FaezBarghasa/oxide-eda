---
okf_version: "0.2"
type: Function
title: sym_editor_toggle_graphic_fill_picker
description: "Toggle the placed graphic's fill picker open / closed. Opening it"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_toggle_graphic_fill_picker
language: rust
---

# sym_editor_toggle_graphic_fill_picker

Toggle the placed graphic's fill picker open / closed. Opening it

## Signature

```rust
impl Oxide { pub(super) fn sym_editor_toggle_graphic_fill_picker(&mut self, idx: usize) -> bool }
```

## Visibility

- `pub(super)`

## Docstring

Toggle the placed graphic's fill picker open / closed. Opening it
closes any local-colour picker. No dirty.

## Source
Lines 116–132 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
