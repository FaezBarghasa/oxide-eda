---
okf_version: "0.2"
type: Function
title: one_undo_reverses_the_whole_multi_pad_rotate
description: "`apply_footprint_primitive_edit` does NOT push a snapshot for Rotate —"
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/one_undo_reverses_the_whole_multi_pad_rotate
language: rust
---

# one_undo_reverses_the_whole_multi_pad_rotate

`apply_footprint_primitive_edit` does NOT push a snapshot for Rotate —

## Signature

```rust
fn one_undo_reverses_the_whole_multi_pad_rotate()
```

## Decorators

- `test`

## Docstring

`apply_footprint_primitive_edit` does NOT push a snapshot for Rotate —
#146 put it on `mutates_footprint_state`'s exemption list — so the arm
pushes exactly ONE itself, gated on a non-empty selection. One Ctrl+Z
has to reverse the whole multi-pad rotate; two would mean the history
got double-stacked, zero would mean it could not be undone at all.
[test]

## Source
Lines 183–216 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| calls | [fixture](/crates/oxide-app/tests/footprint_pad_rotation/fixture.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_rotation/dispatch.md) |
