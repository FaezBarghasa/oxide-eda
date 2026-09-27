---
okf_version: "0.2"
type: Module
title: widgets
description: Shared style / text / button helpers for the Preferences modal.
resource: crates/oxide-app/src/preferences/widgets.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/preferences/widgets
language: rust
---

# widgets

Shared style / text / button helpers for the Preferences modal.

## Docstring

Shared style / text / button helpers for the Preferences modal.

Theme-token-resolved `text::Style` / `button::Style` closures plus
the section-title and horizontal-separator builders. Reached by the
shell (`mod.rs`) and every section submodule via `use super::*` —
re-exported from `mod.rs` at `pub(in crate::preferences)`. Pure code
motion out of the former single-file `preferences` module.

## Relationships

| Type | Target |
|------|--------|
| related | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| related | [text_muted](/crates/oxide-app/src/preferences/widgets/text_muted.md) |
| related | [text_warning](/crates/oxide-app/src/preferences/widgets/text_warning.md) |
| related | [primary_button_style](/crates/oxide-app/src/preferences/widgets/primary_button_style.md) |
| related | [danger_button_style](/crates/oxide-app/src/preferences/widgets/danger_button_style.md) |
| related | [success_button_style](/crates/oxide-app/src/preferences/widgets/success_button_style.md) |
| related | [section_title](/crates/oxide-app/src/preferences/widgets/section_title.md) |
| related | [h_sep](/crates/oxide-app/src/preferences/widgets/h_sep.md) |
| related | [secondary_button_style](/crates/oxide-app/src/preferences/widgets/secondary_button_style.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
