---
okf_version: "0.2"
type: Function
title: is_dark_surface
description: Perceptual-luminance test used to pick the white/black wordmark and
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod/is_dark_surface
language: rust
---

# is_dark_surface

Perceptual-luminance test used to pick the white/black wordmark and

## Signature

```rust
fn is_dark_surface(c: oxide_types::theme::Color) -> bool
```

## Docstring

Perceptual-luminance test used to pick the white/black wordmark and
(later) matching chrome icons. Mirrors the sRGB Y' coefficients so
cyan/green tones don't fool the check like a naive (r+g+b)/3 would.

## Source
Lines 370–376 in `crates/oxide-app/src/menu_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-app/src/menu_bar/mod.md) |
| called_by | [view](/crates/oxide-app/src/menu_bar/view/view.md) |
