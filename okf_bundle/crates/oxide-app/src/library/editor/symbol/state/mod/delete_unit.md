---
okf_version: "0.2"
type: Function
title: delete_unit
description: "Delete sub-part `part` from the symbol: drop its pins, renumber"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/delete_unit
language: rust
---

# delete_unit

Delete sub-part `part` from the symbol: drop its pins, renumber

## Signature

```rust
pub fn delete_unit(sym: &mut Symbol, part: u8) -> u8
```

## Visibility

- `pub`

## Docstring

Delete sub-part `part` from the symbol: drop its pins, renumber
every higher part down by one, and decrement the declared
`part_count`. Part 0 (the "appears on every part" marker) and the
last remaining part are never deleted. Returns the sub-part the
caller should make active after the delete.

## Source
Lines 298–329 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| calls | [max_part_number](/crates/oxide-app/src/library/editor/symbol/state/mod/max_part_number.md) |
| called_by | [delete_unit_out_of_range_leaves_count_unchanged](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_unit_out_of_range_leaves_count_unchanged.md) |
| called_by | [delete_unit_prunes_and_renumbers_graphics](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_unit_prunes_and_renumbers_graphics.md) |
| called_by | [delete_unit_removes_and_renumbers](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_unit_removes_and_renumbers.md) |
| called_by | [apply_symbol_parts](/crates/oxide-app/src/library/editor/symbol/updates/parts/apply_symbol_parts.md) |
