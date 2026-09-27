---
okf_version: "0.2"
type: Function
title: wordmark_tier
description: "Pick the PNG tier that will render closest to 1:1 with device pixels"
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod/wordmark_tier
language: rust
---

# wordmark_tier

Pick the PNG tier that will render closest to 1:1 with device pixels

## Signature

```rust
fn wordmark_tier(scale: f32) -> u8
```

## Docstring

Pick the PNG tier that will render closest to 1:1 with device pixels
at the given OS scale factor. The small slack (`+ 0.05`) absorbs
floating-point jitter — at exactly 1.0 we want the 1× asset, not the
2× downsampled to 96 px.

## Source
Lines 71–80 in `crates/oxide-app/src/menu_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-app/src/menu_bar/mod.md) |
| called_by | [view](/crates/oxide-app/src/menu_bar/view/view.md) |
