---
okf_version: "0.2"
type: Function
title: flip_moves_every_selected_pad_to_the_back_side
description: "The fab-error case: a partial flip leaves some pads on F.* and some"
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/flip_moves_every_selected_pad_to_the_back_side
language: rust
---

# flip_moves_every_selected_pad_to_the_back_side

The fab-error case: a partial flip leaves some pads on F.* and some

## Signature

```rust
fn flip_moves_every_selected_pad_to_the_back_side()
```

## Decorators

- `test`

## Docstring

The fab-error case: a partial flip leaves some pads on F.* and some
on B.*, which is a board that cannot be built.
[test]

## Source
Lines 147–175 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| calls | [fixture](/crates/oxide-app/tests/footprint_pad_rotation/fixture.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_rotation/dispatch.md) |
