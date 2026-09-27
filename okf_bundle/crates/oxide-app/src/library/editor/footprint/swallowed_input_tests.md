---
okf_version: "0.2"
type: Module
title: swallowed_input_tests
description: "GH #599 batch B — numeric inputs whose parse failure used to be"
resource: crates/oxide-app/src/library/editor/footprint/swallowed_input_tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/swallowed_input_tests
language: rust
---

# swallowed_input_tests

GH #599 batch B — numeric inputs whose parse failure used to be

## Docstring

GH #599 batch B — numeric inputs whose parse failure used to be
replaced with a plausible substitute.

Two shapes are covered here:

* a dimensional constraint whose `dimension_input` will not parse
used to end in a bare `if let Some(kind) = new_kind { … }` with
no `else` — no constraint, no message, no highlight, `dirty`
untouched and the canvas cache intact, so even the redraw was
identical;
* the Offset distance and the Fillet radius used to fall back to
0.5 mm on a failed parse, which is worse than a no-op — the
operation SUCCEEDS at a size nobody asked for, and a wrong
fillet radius goes to fabrication.

`1,5` (a comma-decimal keyboard) is the realistic trigger for both.

## Relationships

| Type | Target |
|------|--------|
| related | [line_editor](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/line_editor.md) |
| related | [corner_editor](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/corner_editor.md) |
| related | [constraint_count](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/constraint_count.md) |
| related | [line_count](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/line_count.md) |
| related | [arc_count](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/arc_count.md) |
| related | [distance_constraint_with_comma_decimal_is_reported_not_swallowed](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/distance_constraint_with_comma_decimal_is_reported_not_swallowed.md) |
| related | [angle_constraint_with_unit_suffix_is_reported_not_swallowed](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/angle_constraint_with_unit_suffix_is_reported_not_swallowed.md) |
| related | [distance_constraint_with_a_readable_dimension_still_applies](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/distance_constraint_with_a_readable_dimension_still_applies.md) |
| related | [constraint_that_does_not_match_the_selection_is_reported](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/constraint_that_does_not_match_the_selection_is_reported.md) |
| related | [offset_with_an_unreadable_distance_creates_nothing](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/offset_with_an_unreadable_distance_creates_nothing.md) |
| related | [offset_with_an_empty_distance_still_uses_the_default](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/offset_with_an_empty_distance_still_uses_the_default.md) |
| related | [fillet_with_an_unreadable_radius_creates_nothing](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/fillet_with_an_unreadable_radius_creates_nothing.md) |
| related | [fillet_with_an_empty_radius_still_uses_the_default](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/fillet_with_an_empty_radius_still_uses_the_default.md) |
