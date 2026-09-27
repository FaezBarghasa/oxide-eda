---
okf_version: "0.2"
type: Function
title: next_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on
description: "A staged (>= 3 vertex) Place Polygon click sequence on part 1"
resource: crates/oxide-app/src/library/editor/symbol/updates/parts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/parts/next_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on
language: rust
---

# next_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on

A staged (>= 3 vertex) Place Polygon click sequence on part 1

## Signature

```rust
fn next_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on()
```

## Decorators

- `test`

## Docstring

A staged (>= 3 vertex) Place Polygon click sequence on part 1
commits BEFORE `NextPart` switches the active part, so the
polygon lands on part 1 (the part it was actually drawn on),
not silently landing invisible on part 2 by reading
`active_part` at some later commit point.
[test]

## Source
Lines 107–129 in `crates/oxide-app/src/library/editor/symbol/updates/parts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parts](/crates/oxide-app/src/library/editor/symbol/updates/parts.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/parts/new_editor.md) |
| calls | [apply_symbol_parts](/crates/oxide-app/src/library/editor/symbol/updates/parts/apply_symbol_parts.md) |
