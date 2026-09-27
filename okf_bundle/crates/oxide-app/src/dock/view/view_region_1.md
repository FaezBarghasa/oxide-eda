---
okf_version: "0.2"
type: Function
title: view_region
resource: crates/oxide-app/src/dock/view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/dock/view/view_region_1
language: rust
---

# view_region

## Signature

```rust
pub fn view_region(
        &'a self,
        position: PanelPosition,
        ctx: &'a panels::PanelContext,
        library: &'a crate::library::LibraryState,
    ) -> Element<'a, DockMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 20–252 in `crates/oxide-app/src/dock/view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/dock/view.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [svg_icon](/crates/oxide-app/src/dock/view/svg_icon.md) |
| calls | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |
