---
okf_version: "0.2"
type: Function
title: single_arc_selection_is_eligible_and_self_closes
description: "A single selected `Arc` IS eligible — a sufficiently large"
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/single_arc_selection_is_eligible_and_self_closes
language: rust
---

# single_arc_selection_is_eligible_and_self_closes

A single selected `Arc` IS eligible — a sufficiently large

## Signature

```rust
fn single_arc_selection_is_eligible_and_self_closes()
```

## Decorators

- `test`

## Docstring

A single selected `Arc` IS eligible — a sufficiently large
sweep can legitimately self-close via its own tiny chord gap —
and joins successfully on its own.
[test]

## Source
Lines 555–577 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/new_editor.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
