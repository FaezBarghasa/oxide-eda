---
okf_version: "0.2"
type: Function
title: hit_test_graphic_handle_finds_polygon_vertex_when_selected
description: "A `PolygonVertex` handle only hit-tests when its polygon is the"
resource: crates/oxide-app/src/library/editor/symbol/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_graphic_handle_finds_polygon_vertex_when_selected
language: rust
---

# hit_test_graphic_handle_finds_polygon_vertex_when_selected

A `PolygonVertex` handle only hit-tests when its polygon is the

## Signature

```rust
fn hit_test_graphic_handle_finds_polygon_vertex_when_selected()
```

## Decorators

- `test`

## Docstring

A `PolygonVertex` handle only hit-tests when its polygon is the
currently-selected graphic — otherwise a click near one of its
(possibly many, tessellated-arc-side) vertices would grab an
invisible handle instead of falling through to `hit_test`'s body
selection.
[test]

## Source
Lines 711–716 in `crates/oxide-app/src/library/editor/symbol/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/symbol/state/tests.md) |
| calls | [polygon_symbol](/crates/oxide-app/src/library/editor/symbol/state/tests/polygon_symbol.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| calls | [hit_test_graphic_handle](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_handle.md) |
