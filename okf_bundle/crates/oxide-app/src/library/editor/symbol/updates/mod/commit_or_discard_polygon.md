---
okf_version: "0.2"
type: Function
title: commit_or_discard_polygon
description: "Commit `editor.polygon_vertices` (the Place Polygon click-collect"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/commit_or_discard_polygon
language: rust
---

# commit_or_discard_polygon

Commit `editor.polygon_vertices` (the Place Polygon click-collect

## Signature

```rust
pub(super) fn commit_or_discard_polygon(editor: &mut SymEditor)
```

## Visibility

- `pub(super)`

## Docstring

Commit `editor.polygon_vertices` (the Place Polygon click-collect
stash) if it holds a valid closed ring, else silently discard it.
Always empties the stash. Shared by the `PolygonCommit` message
handler (close-by-click / double-click / Enter) and the `SetTool`
handler's synchronous "leaving Place Polygon" flush — see
`apply_symbol_ui`'s `SetTool` arm for why that flush has to be
synchronous rather than deferred to a later event.

## Source
Lines 147–160 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [normalize_polygon_ring](/crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_polygon_ring.md) |
| calls | [push_graphic](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_graphic.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
| called_by | [apply_symbol_parts](/crates/oxide-app/src/library/editor/symbol/updates/parts/apply_symbol_parts.md) |
| called_by | [apply_symbol_ui](/crates/oxide-app/src/library/editor/symbol/updates/ui/apply_symbol_ui.md) |
