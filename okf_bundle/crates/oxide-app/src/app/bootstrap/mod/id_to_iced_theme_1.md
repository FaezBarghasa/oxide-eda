---
okf_version: "0.2"
type: Function
title: id_to_iced_theme
description: "Map a ThemeId to an iced::Theme with a properly tuned palette."
resource: crates/oxide-app/src/app/bootstrap/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/mod/id_to_iced_theme_1
language: rust
---

# id_to_iced_theme

Map a ThemeId to an iced::Theme with a properly tuned palette.

## Signature

```rust
fn id_to_iced_theme(
        id: ThemeId,
        custom: Option<&oxide_types::theme::CustomThemeFile>,
    ) -> Theme
```

## Docstring

Map a ThemeId to an iced::Theme with a properly tuned palette.

## Source
Lines 60–133 in `crates/oxide-app/src/app/bootstrap/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bootstrap](/crates/oxide-app/src/app/bootstrap/mod.md) |
