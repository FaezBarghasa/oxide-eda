---
okf_version: "0.2"
type: Module
title: errors
description: "Validation + the degenerate-input error taxonomy for `split_line`."
resource: crates/oxide-sketch/src/split/tests/errors.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/tests/errors
language: rust
---

# errors

Validation + the degenerate-input error taxonomy for `split_line`.

## Docstring

Validation + the degenerate-input error taxonomy for `split_line`.
Every case here must return `Err` and leave `sketch` byte-for-byte
unchanged.

## Relationships

| Type | Target |
|------|--------|
| related | [ulp_absorbed_mid_point_is_rejected](/crates/oxide-sketch/src/split/tests/errors/ulp_absorbed_mid_point_is_rejected.md) |
| related | [mid_too_close_to_endpoint_on_a_long_line_is_rejected](/crates/oxide-sketch/src/split/tests/errors/mid_too_close_to_endpoint_on_a_long_line_is_rejected.md) |
| related | [a_realistic_close_to_end_split_still_succeeds](/crates/oxide-sketch/src/split/tests/errors/a_realistic_close_to_end_split_still_succeeds.md) |
| related | [line_between_min_and_2x_min_is_degenerate_at_every_t](/crates/oxide-sketch/src/split/tests/errors/line_between_min_and_2x_min_is_degenerate_at_every_t.md) |
| related | [line_just_over_2x_min_still_splits_at_its_midpoint](/crates/oxide-sketch/src/split/tests/errors/line_just_over_2x_min_still_splits_at_its_midpoint.md) |
| related | [too_close_to_endpoint_line_always_has_a_better_t](/crates/oxide-sketch/src/split/tests/errors/too_close_to_endpoint_line_always_has_a_better_t.md) |
| related | [nan_endpoint_coordinate_is_rejected_not_silently_propagated](/crates/oxide-sketch/src/split/tests/errors/nan_endpoint_coordinate_is_rejected_not_silently_propagated.md) |
| related | [infinite_endpoint_coordinate_is_rejected](/crates/oxide-sketch/src/split/tests/errors/infinite_endpoint_coordinate_is_rejected.md) |
| related | [missing_line_id_returns_err](/crates/oxide-sketch/src/split/tests/errors/missing_line_id_returns_err.md) |
| related | [wrong_kind_returns_err](/crates/oxide-sketch/src/split/tests/errors/wrong_kind_returns_err.md) |
| related | [zero_length_line_returns_err](/crates/oxide-sketch/src/split/tests/errors/zero_length_line_returns_err.md) |
| related | [t_at_or_beyond_an_endpoint_returns_err](/crates/oxide-sketch/src/split/tests/errors/t_at_or_beyond_an_endpoint_returns_err.md) |
| related | [t_within_min_segment_len_of_an_endpoint_returns_err](/crates/oxide-sketch/src/split/tests/errors/t_within_min_segment_len_of_an_endpoint_returns_err.md) |
| related | [t_nan_or_infinite_returns_err](/crates/oxide-sketch/src/split/tests/errors/t_nan_or_infinite_returns_err.md) |
| related | [unresolvable_endpoint_returns_err](/crates/oxide-sketch/src/split/tests/errors/unresolvable_endpoint_returns_err.md) |
| related | [failed_split_leaves_sketch_byte_identical](/crates/oxide-sketch/src/split/tests/errors/failed_split_leaves_sketch_byte_identical.md) |
