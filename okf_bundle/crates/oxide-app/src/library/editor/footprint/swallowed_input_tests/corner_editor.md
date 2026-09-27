---
okf_version: "0.2"
type: Function
title: corner_editor
description: Editor holding two Lines meeting at the origin — the shape the
resource: crates/oxide-app/src/library/editor/footprint/swallowed_input_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/corner_editor
language: rust
---

# corner_editor

Editor holding two Lines meeting at the origin — the shape the

## Signature

```rust
fn corner_editor() -> (
        crate::app::FootprintEditorState,
        SketchEntityId,
        SketchEntityId,
    )
```

## Docstring

Editor holding two Lines meeting at the origin — the shape the
Fillet tool needs, and two Lines for the Angle constraint.
Returns the editor plus the east-going and north-going Line ids.

## Source
Lines 66–112 in `crates/oxide-app/src/library/editor/footprint/swallowed_input_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [swallowed_input_tests](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests.md) |
| called_by | [angle_constraint_with_unit_suffix_is_reported_not_swallowed](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/angle_constraint_with_unit_suffix_is_reported_not_swallowed.md) |
| called_by | [fillet_with_an_empty_radius_still_uses_the_default](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/fillet_with_an_empty_radius_still_uses_the_default.md) |
| called_by | [fillet_with_an_unreadable_radius_creates_nothing](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/fillet_with_an_unreadable_radius_creates_nothing.md) |
