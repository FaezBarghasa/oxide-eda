---
okf_version: "0.2"
type: Function
title: dock_zone_highlight
description: Translucent highlight overlay shown on dock regions when a floating panel
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/dock_zone_highlight
language: rust
---

# dock_zone_highlight

Translucent highlight overlay shown on dock regions when a floating panel

## Signature

```rust
pub fn dock_zone_highlight(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Translucent highlight overlay shown on dock regions when a floating panel
is dragged near a window edge.

## Source
Lines 308–321 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [dock_drag_zone_overlay](/crates/oxide-app/src/app/view/overlays/mod/dock_drag_zone_overlay.md) |
