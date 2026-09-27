---
okf_version: "0.2"
type: Function
title: graphic_is_selected
description: "Whether the graphic at `idx` is part of `sel` — single source of"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/graphic_is_selected
language: rust
---

# graphic_is_selected

Whether the graphic at `idx` is part of `sel` — single source of

## Signature

```rust
pub fn graphic_is_selected(sel: &Option<SymbolSelection>, idx: usize) -> bool
```

## Visibility

- `pub`

## Docstring

Whether the graphic at `idx` is part of `sel` — single source of
truth for "is this graphic currently selected," shared by the
canvas draw path (selection-colour + resize-handle visibility) and
the hit-test path (scoping `PolygonVertex` handle hit-testing to
the selected polygon only — see `hit_test_graphic_handle`'s doc
comment) so the two can never disagree about which graphic is
selected.

## Source
Lines 344–353 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [is_graphic_selected](/crates/oxide-app/src/library/editor/symbol/canvas/mod/is_graphic_selected.md) |
| called_by | [hit_test_graphic_handle](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_handle.md) |
