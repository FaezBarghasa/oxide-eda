---
okf_version: "0.2"
type: Function
title: hit_test_graphic_handle_ignores_polygon_vertex_when_not_selected
description: "An unselected polygon's vertices don't hit-test at all — a click"
resource: crates/oxide-app/src/library/editor/symbol/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_graphic_handle_ignores_polygon_vertex_when_not_selected
language: rust
---

# hit_test_graphic_handle_ignores_polygon_vertex_when_not_selected

An unselected polygon's vertices don't hit-test at all — a click

## Signature

```rust
fn hit_test_graphic_handle_ignores_polygon_vertex_when_not_selected()
```

## Decorators

- `test`

## Docstring

An unselected polygon's vertices don't hit-test at all — a click
on its body must select the shape, not silently grab a vertex
handle the user can't even see (the draw path only renders
handles for the selected graphic).
[test]

## Source
Lines 723–735 in `crates/oxide-app/src/library/editor/symbol/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/symbol/state/tests.md) |
| calls | [polygon_symbol](/crates/oxide-app/src/library/editor/symbol/state/tests/polygon_symbol.md) |
| calls | [hit_test_graphic_handle](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_handle.md) |
