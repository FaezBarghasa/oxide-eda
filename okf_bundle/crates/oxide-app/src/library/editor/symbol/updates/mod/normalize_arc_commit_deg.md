---
okf_version: "0.2"
type: Function
title: normalize_arc_commit_deg
description: "Normalise an about-to-be-stored `SymbolGraphicKind::Arc`'s"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_arc_commit_deg
language: rust
---

# normalize_arc_commit_deg

Normalise an about-to-be-stored `SymbolGraphicKind::Arc`'s

## Signature

```rust
pub(super) fn normalize_arc_commit_deg(start_deg: f64, end_deg: f64) -> (f64, f64)
```

## Visibility

- `pub(super)`

## Docstring

Normalise an about-to-be-stored `SymbolGraphicKind::Arc`'s
`start_deg`/`end_deg` into this codebase's CCW-wraparound
convention. The three-click placement gesture's third click can
produce `end_deg < start_deg` two different ways: a genuinely CW
drag (the cursor moved backwards past the start angle, so the
unwrapped tracked end angle went negative relative to start — e.g.
`start: 30, end: -60`), or simply because `start_deg` is a raw
`atan2` result while `end_deg` is separately unwrapped and the two
never got reconciled into a common `[0, 360)` frame. Either way,
under the CCW-wraparound rule `end_deg < start_deg` is read as
"the long way around" — the opposite of the short arc the
placement preview showed (see `draw_arc_preview`, which builds its
ghost from the exact same CCW-wraparound sweep this function's
callers store).

A thin wrapper over `oxide_library::normalize_arc_endpoints_deg`
— that function's doc comment has the full swap-vs-`rem_euclid`
rationale, shared verbatim with `SymbolFile::from_toml_str`'s
legacy-arc load migration (which reuses the exact same function
rather than duplicating this formula a third time; oxide-library
must not depend on oxide-app, so the shared implementation lives
there and this crate calls into it, not the reverse).

## Source
Lines 280–282 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [normalize_arc_endpoints_deg](/crates/oxide-library/src/primitive/symbol/mod/normalize_arc_endpoints_deg.md) |
| called_by | [draw_arc_preview](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_arc_preview.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
