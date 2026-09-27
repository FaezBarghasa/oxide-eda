---
okf_version: "0.2"
type: Function
title: place_move_button
resource: crates/oxide-app/src/library/editor/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/tests/place_move_button
language: rust
---

# place_move_button

## Signature

```rust
fn place_move_button(
    editor: crate::app::FootprintEditorState,
) -> oxide_widgets::active_bar::ActiveBarItem<crate::library::messages::LibraryMessage>
```

## Source
Lines 249–258 in `crates/oxide-app/src/library/editor/footprint/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/tests.md) |
| calls | [theme_tokens](/crates/oxide-types/src/theme/theme_tokens.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [place_move_button_left_click_arms_select_tool](/crates/oxide-app/src/library/editor/footprint/tests/place_move_button_left_click_arms_select_tool.md) |
