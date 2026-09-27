---
okf_version: "0.2"
type: Function
title: dropdown_min_width
description: Pinned column width per dropdown menu.
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/dropdown_min_width
language: rust
---

# dropdown_min_width

Pinned column width per dropdown menu.

## Signature

```rust
fn dropdown_min_width(menu: ActiveBarMenu) -> Option<f32>
```

## Docstring

Pinned column width per dropdown menu.

`view_dropdown` wraps the items column in a `Length::Fixed(W)`
container using this value. That bound lets each item set
`button.width(Length::Fill)` (so the hover background covers the
full row) without `Fill` propagating to the viewport — which is the
`Length::Fill`-inside-`Length::Shrink` trap iced 0.14 falls into.

Widths are sized to the longest label in each menu (Roboto @ 13 +
28 px icon column + 24 px button padding + a small safety margin).
`Filter` returns `None` because its chip wrap layout already drives
its own width.

## Source
Lines 833–866 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
| called_by | [view_dropdown](/crates/oxide-app/src/active_bar/dropdown/view_dropdown.md) |
