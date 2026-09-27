---
okf_version: "0.2"
type: Function
title: pin_rotation_matches_oxide_output_pin_direction_convention
description: "Ground-truth convention test: ties `pin_rotation_deg`'s output to the"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin_rotation_matches_oxide_output_pin_direction_convention
language: rust
---

# pin_rotation_matches_oxide_output_pin_direction_convention

Ground-truth convention test: ties `pin_rotation_deg`'s output to the

## Signature

```rust
fn pin_rotation_matches_oxide_output_pin_direction_convention()
```

## Decorators

- `test`

## Docstring

Ground-truth convention test: ties `pin_rotation_deg`'s output to the
convention the actual downstream renderer uses, so the two mappings
can't silently drift apart (the other rotation test above only
checks self-consistency with the formula in this crate).

`crates/oxide-output/src/svg/symbols.rs`'s private `pin_direction(pin:
&Pin) -> (f64, f64)` is the real consumer that turns
`LibPin.pin.rotation` into a draw direction: `0 => (1.0, 0.0)`, `90 =>
(0.0, 1.0)`, `180 => (-1.0, 0.0)`, `270 => (0.0, -1.0)`. `oxide-library`
does not (and must not, per the workspace's dependency direction —
`oxide-output` depends on `oxide_types`/`oxide-library`, never the
reverse) depend on `oxide-output`, so that function cannot be called
from this test. Instead this re-derives the identical formula inline
and asserts the resulting unit vector against the direction each
`PinOrientation` is documented to mean: `Right -> +x`, `Up -> +y`,
`Left -> -x`, `Down -> -y`.
[test]

## Source
Lines 154–190 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol_tests](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.md) |
| calls | [pin_direction_convention](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin_direction_convention.md) |
