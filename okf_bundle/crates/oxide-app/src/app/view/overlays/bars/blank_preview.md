---
okf_version: "0.2"
type: Function
title: blank_preview
description: A print preview with no pages — enough to set the state flag the
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/blank_preview
language: rust
---

# blank_preview

A print preview with no pages — enough to set the state flag the

## Signature

```rust
fn blank_preview() -> PreviewState
```

## Docstring

A print preview with no pages — enough to set the state flag the
blocking predicate reads, which is all these tests care about.

## Source
Lines 746–762 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
| called_by | [a_detached_print_preview_stops_blocking_the_main_windows_overlay_stack](/crates/oxide-app/src/app/view/overlays/bars/a_detached_print_preview_stops_blocking_the_main_windows_overlay_stack.md) |
| called_by | [the_painter_and_the_esc_ladder_agree_about_a_detached_preview](/crates/oxide-app/src/app/view/overlays/bars/the_painter_and_the_esc_ladder_agree_about_a_detached_preview.md) |
