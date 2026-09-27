---
okf_version: "0.2"
type: Function
title: render_section
description: "---------------------------------------------------------------------------"
resource: crates/oxide-widgets/src/status_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/status_bar/render_section
language: rust
---

# render_section

---------------------------------------------------------------------------

## Signature

```rust
fn render_section(
    section: &StatusSection,
    index: usize,
    text_color: iced::Color,
    muted_color: iced::Color,
    success_color: iced::Color,
) -> Element<'a, StatusBarMsg>
```

## Type Parameters

- `'a`

## Docstring

---------------------------------------------------------------------------
Section rendering
---------------------------------------------------------------------------

## Source
Lines 130–175 in `crates/oxide-widgets/src/status_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [status_bar](/crates/oxide-widgets/src/status_bar.md) |
| called_by | [status_bar](/crates/oxide-widgets/src/status_bar/status_bar.md) |
