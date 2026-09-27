---
okf_version: "0.2"
type: Function
title: line_editor
description: "Editor holding one horizontal Line from (0, 0) to (10, 0) plus"
resource: crates/oxide-app/src/library/editor/footprint/swallowed_input_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/line_editor
language: rust
---

# line_editor

Editor holding one horizontal Line from (0, 0) to (10, 0) plus

## Signature

```rust
fn line_editor() -> (
        crate::app::FootprintEditorState,
        SketchEntityId,
        SketchEntityId,
        SketchEntityId,
    )
```

## Docstring

Editor holding one horizontal Line from (0, 0) to (10, 0) plus
its two endpoints. Returns the endpoint ids and the Line id.

## Source
Lines 33–61 in `crates/oxide-app/src/library/editor/footprint/swallowed_input_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [swallowed_input_tests](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests.md) |
| called_by | [constraint_that_does_not_match_the_selection_is_reported](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/constraint_that_does_not_match_the_selection_is_reported.md) |
| called_by | [distance_constraint_with_a_readable_dimension_still_applies](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/distance_constraint_with_a_readable_dimension_still_applies.md) |
| called_by | [distance_constraint_with_comma_decimal_is_reported_not_swallowed](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/distance_constraint_with_comma_decimal_is_reported_not_swallowed.md) |
| called_by | [offset_with_an_empty_distance_still_uses_the_default](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/offset_with_an_empty_distance_still_uses_the_default.md) |
| called_by | [offset_with_an_unreadable_distance_creates_nothing](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/offset_with_an_unreadable_distance_creates_nothing.md) |
