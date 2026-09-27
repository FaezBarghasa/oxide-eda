---
okf_version: "0.2"
type: Function
title: estimate_text_width
description: "Estimate the rendered pixel width of a string at `font_size`."
resource: crates/oxide-app/src/dock/view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/dock/view/estimate_text_width
language: rust
---

# estimate_text_width

Estimate the rendered pixel width of a string at `font_size`.

## Signature

```rust
fn estimate_text_width(s: &str, font_size: f32) -> f32
```

## Docstring

Estimate the rendered pixel width of a string at `font_size`.

Ratios are calibrated for Segoe UI (Windows default / Iced default).
Grouped by measured glyph-width bands so every panel label gets
near-identical visual padding after center-alignment.

## Source
Lines 412–440 in `crates/oxide-app/src/dock/view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/dock/view.md) |
| called_by | [view_rail](/crates/oxide-app/src/dock/view/view_rail.md) |
