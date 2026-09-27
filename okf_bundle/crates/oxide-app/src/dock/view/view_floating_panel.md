---
okf_version: "0.2"
type: Function
title: view_floating_panel
description: Render a single floating panel as an overlay element.
resource: crates/oxide-app/src/dock/view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/dock/view/view_floating_panel
language: rust
---

# view_floating_panel

Render a single floating panel as an overlay element.

## Signature

```rust
impl DockArea { pub fn view_floating_panel(
        &'a self,
        idx: usize,
        ctx: &'a panels::PanelContext,
        library: &'a crate::library::LibraryState,
    ) -> Option<Element<'a, DockMessage>> }
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render a single floating panel as an overlay element.

## Source
Lines 345–399 in `crates/oxide-app/src/dock/view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/dock/view.md) |
| calls | [floating_title_bar](/crates/oxide-app/src/styles/floating_title_bar.md) |
| calls | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |
| calls | [floating_panel_shadow](/crates/oxide-app/src/styles/floating_panel_shadow.md) |
