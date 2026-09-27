---
okf_version: "0.2"
type: Function
title: test_prefs
description: "Minimal render settings for the tests below. `update_pending_fit`"
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/test_prefs
language: rust
---

# test_prefs

Minimal render settings for the tests below. `update_pending_fit`

## Signature

```rust
fn test_prefs(
        overrides: &std::collections::HashMap<uuid::Uuid, oxide_types::theme::Color>,
    ) -> CanvasViewPrefs<'_>
```

## Docstring

Minimal render settings for the tests below. `update_pending_fit`
only touches the camera, so the values here just have to exist —
#631 moved the method onto the borrowing view, which needs both
halves to be constructed.

## Source
Lines 759–777 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
| called_by | [a_consumed_fit_does_not_re_apply](/crates/oxide-app/src/canvas/mod/a_consumed_fit_does_not_re_apply.md) |
| called_by | [a_pending_fit_reaches_the_camera_in_one_hop](/crates/oxide-app/src/canvas/mod/a_pending_fit_reaches_the_camera_in_one_hop.md) |
