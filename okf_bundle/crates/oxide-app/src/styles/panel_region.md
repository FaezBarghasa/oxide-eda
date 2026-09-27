---
okf_version: "0.2"
type: Function
title: panel_region
description: Panel region (left/right/bottom docks)
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/panel_region
language: rust
---

# panel_region

Panel region (left/right/bottom docks)

## Signature

```rust
pub fn panel_region(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Panel region (left/right/bottom docks)

## Source
Lines 22–36 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_dock_panel](/crates/oxide-app/src/app/view/chrome/view_dock_panel.md) |
| called_by | [view_dock_panel_h](/crates/oxide-app/src/app/view/chrome/view_dock_panel_h.md) |
| called_by | [view_center](/crates/oxide-app/src/app/view/mod/view_center.md) |
