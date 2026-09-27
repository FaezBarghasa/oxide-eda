---
okf_version: "0.2"
type: Function
title: add_arc_commit_stores_swapped_endpoints_for_a_cw_drag
description: "`AddArc` commits a CW-dragged placement (`end_deg < start_deg`)"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/add_arc_commit_stores_swapped_endpoints_for_a_cw_drag
language: rust
---

# add_arc_commit_stores_swapped_endpoints_for_a_cw_drag

`AddArc` commits a CW-dragged placement (`end_deg < start_deg`)

## Signature

```rust
fn add_arc_commit_stores_swapped_endpoints_for_a_cw_drag()
```

## Decorators

- `test`

## Docstring

`AddArc` commits a CW-dragged placement (`end_deg < start_deg`)
with its endpoints swapped, so the graphic that lands in
`Symbol::graphics` — not just the intermediate helper — stores
the CCW-wraparound form of the short arc the preview showed.
[test]

## Source
Lines 850–877 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/mod/new_editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
