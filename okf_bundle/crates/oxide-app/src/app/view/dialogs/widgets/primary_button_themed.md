---
okf_version: "0.2"
type: Function
title: primary_button_themed
description: "Theme-aware primary button. Pass `Some(accent)` to use the"
resource: crates/oxide-app/src/app/view/dialogs/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/widgets/primary_button_themed
language: rust
---

# primary_button_themed

Theme-aware primary button. Pass `Some(accent)` to use the

## Signature

```rust
pub(super) fn primary_button_themed(
    label: &str,
    message: Option<Message>,
    border: Color,
    accent: Option<Color>,
) -> Element<'_, Message>
```

## Visibility

- `pub(super)`

## Docstring

Theme-aware primary button. Pass `Some(accent)` to use the
theme's accent colour as the button bg (Altium-amber on Oxide,
cyan on Alp Lab, etc.). Pass `None` to fall back to the legacy
hardcoded blue (existing call sites that haven't been migrated).

## Source
Lines 227–260 in `crates/oxide-app/src/app/view/dialogs/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/app/view/dialogs/widgets.md) |
| called_by | [primary_button](/crates/oxide-app/src/app/view/dialogs/widgets/primary_button.md) |
