---
okf_version: "0.2"
type: Function
title: align_selected_to_grid_with_already_aligned_pin_stays_clean
description: "#477 — a selection that resolves to a real pin is"
resource: crates/oxide-app/src/library/editor/symbol/updates/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_with_already_aligned_pin_stays_clean
language: rust
---

# align_selected_to_grid_with_already_aligned_pin_stays_clean

#477 — a selection that resolves to a real pin is

## Signature

```rust
fn align_selected_to_grid_with_already_aligned_pin_stays_clean()
```

## Decorators

- `test`

## Docstring

#477 — a selection that resolves to a real pin is
"alignable-shaped", but when that pin already sits exactly on
the 1.27 mm grid, snapping it moves nothing. `changed` must be
delta-based (did a coordinate actually move), not existence-
based (did the index resolve to a pin) — otherwise this no-op
still pushes an undo snapshot and clears the redo stack, same
bug as the empty-`All` case above just reached through a
different selection shape. Seed one redo entry and assert
neither stack moves.
[test]

## Source
Lines 255–278 in `crates/oxide-app/src/library/editor/symbol/updates/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/symbol/updates/transform.md) |
| calls | [add_pin](/crates/oxide-app/src/library/editor/symbol/state/mod/add_pin.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| calls | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
