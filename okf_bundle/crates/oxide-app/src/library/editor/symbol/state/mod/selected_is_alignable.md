---
okf_version: "0.2"
type: Function
title: selected_is_alignable
description: "Whether [`align_selected_to_grid`] would actually snap anything for"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/selected_is_alignable
language: rust
---

# selected_is_alignable

Whether [`align_selected_to_grid`] would actually snap anything for

## Signature

```rust
pub fn selected_is_alignable(selected: &Option<SymbolSelection>) -> bool
```

## Visibility

- `pub`

## Docstring

Whether [`align_selected_to_grid`] would actually snap anything for
this selection. Unlike [`selected_is_deletable`], `All` IS eligible
here — aligning to grid never destroys data the way a whole-symbol
Delete would, so there's no accidental-wipe concern to guard
against. Only `None`/`Field` are true no-ops (a field has no
canvas position of its own yet). Callers gate the undo snapshot on
this so a no-op Align To Grid press stays clean, mirroring how
[`selected_is_deletable`] gates Delete.

## Source
Lines 537–539 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
