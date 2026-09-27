---
okf_version: "0.2"
type: Function
title: place_move_button_left_click_arms_select_tool
description: "Issue #375 — the Place / Move active-bar button's left-click must"
resource: crates/oxide-app/src/library/editor/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/tests/place_move_button_left_click_arms_select_tool
language: rust
---

# place_move_button_left_click_arms_select_tool

Issue #375 — the Place / Move active-bar button's left-click must

## Signature

```rust
fn place_move_button_left_click_arms_select_tool()
```

## Decorators

- `test`

## Docstring

Issue #375 — the Place / Move active-bar button's left-click must
arm PadsTool::Select (a footprint has no separate move tool; pad
movement is drag-under-Select — see `active_bar_dropdowns.rs`'s
`place_entries`), not fall through to the `ActiveBarStub` no-op.

Pinned by POSITION, not by tooltip text: `dropdown_trigger_items`
puts Filter, Snap, Place, Select, Align, Body3d, Text, Shapes in
that exact order (the same order `dropdown_x_offset` documents and
depends on for dropdown placement), so the Place/Move button is
always index 2. A `tooltip.contains("Move")` assertion would
couple this test to prose the tooltip-wording fix itself rewrites.
[test]

## Source
Lines 189–205 in `crates/oxide-app/src/library/editor/footprint/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/tests.md) |
| calls | [place_move_button](/crates/oxide-app/src/library/editor/footprint/tests/place_move_button.md) |
| calls | [default_editor](/crates/oxide-app/src/library/editor/footprint/tests/default_editor.md) |
