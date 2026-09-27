---
okf_version: "0.2"
type: Function
title: new_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on
description: "Same flush, but for `PrevPart` / `NewPart` / `RemovePart` too —"
resource: crates/oxide-app/src/library/editor/symbol/updates/parts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/parts/new_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on
language: rust
---

# new_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on

Same flush, but for `PrevPart` / `NewPart` / `RemovePart` too —

## Signature

```rust
fn new_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on()
```

## Decorators

- `test`

## Docstring

Same flush, but for `PrevPart` / `NewPart` / `RemovePart` too —
all four multi-part messages share the same top-of-function
flush, not just `NextPart`.
[test]

## Source
Lines 135–153 in `crates/oxide-app/src/library/editor/symbol/updates/parts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parts](/crates/oxide-app/src/library/editor/symbol/updates/parts.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/parts/new_editor.md) |
| calls | [apply_symbol_parts](/crates/oxide-app/src/library/editor/symbol/updates/parts/apply_symbol_parts.md) |
