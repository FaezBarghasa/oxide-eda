---
okf_version: "0.2"
type: Function
title: svg_icon
description: "Wrap a themed SVG handle into a 10×10 `Svg` widget — matches the"
resource: crates/oxide-app/src/dock/view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/dock/view/svg_icon
language: rust
---

# svg_icon

Wrap a themed SVG handle into a 10×10 `Svg` widget — matches the

## Signature

```rust
fn svg_icon(handle: svg::Handle) -> iced::widget::Svg<'static>
```

## Docstring

Wrap a themed SVG handle into a 10×10 `Svg` widget — matches the
dimensions of the old LazyLock-based helper. Kept as a free
function so every call site reads `svg_icon(icons::icon_close(tid))`
without extra boilerplate.

## Source
Lines 15–17 in `crates/oxide-app/src/dock/view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/dock/view.md) |
| called_by | [view_rail](/crates/oxide-app/src/dock/view/view_rail.md) |
| called_by | [view_region](/crates/oxide-app/src/dock/view/view_region.md) |
