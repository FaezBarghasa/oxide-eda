---
okf_version: "0.2"
type: Function
title: close_x_button
description: Compact X close button for borderless modal headers. Matches the
resource: crates/oxide-app/src/app/view/dialogs/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/widgets/close_x_button
language: rust
---

# close_x_button

Compact X close button for borderless modal headers. Matches the

## Signature

```rust
pub(crate) fn close_x_button(
    message: Message,
    theme_id: oxide_types::theme::ThemeId,
    _text_color: Color,
) -> Element<'static, Message>
```

## Visibility

- `pub(crate)`

## Docstring

Compact X close button for borderless modal headers. Matches the
main-window chrome close (`view/mod.rs::view_main_window_chrome`):
no border, fully transparent at rest, Windows-native red bg + white
icon on hover. The `_text_color` argument is kept for API
compatibility with existing call sites — it is intentionally
ignored, hence the leading underscore.

## Source
Lines 88–141 in `crates/oxide-app/src/app/view/dialogs/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/app/view/dialogs/widgets.md) |
