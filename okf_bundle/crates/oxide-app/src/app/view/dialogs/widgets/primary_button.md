---
okf_version: "0.2"
type: Function
title: primary_button
resource: crates/oxide-app/src/app/view/dialogs/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/widgets/primary_button
language: rust
---

# primary_button

## Signature

```rust
pub(super) fn primary_button(
    label: &str,
    message: Option<Message>,
    border: Color,
) -> Element<'_, Message>
```

## Visibility

- `pub(super)`

## Source
Lines 215–221 in `crates/oxide-app/src/app/view/dialogs/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/app/view/dialogs/widgets.md) |
| calls | [primary_button_themed](/crates/oxide-app/src/app/view/dialogs/widgets/primary_button_themed.md) |
