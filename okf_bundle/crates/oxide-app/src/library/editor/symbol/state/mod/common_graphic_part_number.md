---
okf_version: "0.2"
type: Function
title: common_graphic_part_number
description: "The single `part_number` every graphic `indices` names shares, or"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/common_graphic_part_number
language: rust
---

# common_graphic_part_number

The single `part_number` every graphic `indices` names shares, or

## Signature

```rust
pub fn common_graphic_part_number(sym: &Symbol, indices: &[usize]) -> Option<u8>
```

## Visibility

- `pub`

## Docstring

The single `part_number` every graphic `indices` names shares, or
`None` if `indices` is empty, any index is stale, or they don't all
agree. A join across a shared (part 0) shape and one of a unit's
own shapes must never silently rescope the shared shape onto just
that one unit — shared geometry is admitted by hit-test and
box-select on every unit (see `graphic_on_part`), so a non-uniform
result disqualifies the whole selection rather than picking a part
number to overwrite the other sources with.

## Source
Lines 465–471 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [selection_is_join_eligible](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_is_join_eligible.md) |
| called_by | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
