---
okf_version: "0.2"
type: Function
title: rotate_selected
description: Rotate the selected entity by 90°.
resource: crates/oxide-app/src/library/editor/symbol/state/rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected
language: rust
---

# rotate_selected

Rotate the selected entity by 90°.

## Signature

```rust
pub fn rotate_selected(sym: &mut Symbol, sel: Option<SymbolSelection>, clockwise: bool)
```

## Visibility

- `pub`

## Docstring

Rotate the selected entity by 90°.

Pins rotate in place around their own center (orientation only),
while graphics rotate around world origin to preserve the existing
symbol-body rotate behavior.
`clockwise = true` rotates CW, `false` rotates CCW.

## Source
Lines 11–13 in `crates/oxide-app/src/library/editor/symbol/state/rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation](/crates/oxide-app/src/library/editor/symbol/state/rotation.md) |
| calls | [rotate_selected_with_pivot](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected_with_pivot.md) |
| called_by | [rotate_selected_rotates_pin_orientation_in_place](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_rotates_pin_orientation_in_place.md) |
| called_by | [rotate_selected_rotates_rectangle_clockwise_around_origin](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_rotates_rectangle_clockwise_around_origin.md) |
| called_by | [rotated_wraparound_arc_hit_test_and_draw_sweep_agree](/crates/oxide-app/src/library/editor/symbol/state/tests/rotated_wraparound_arc_hit_test_and_draw_sweep_agree.md) |
