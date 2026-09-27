---
okf_version: "0.2"
type: Function
title: view_rail
resource: crates/oxide-app/src/dock/view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/dock/view/view_rail
language: rust
---

# view_rail

## Signature

```rust
impl DockArea { fn view_rail(
        &'a self,
        position: PanelPosition,
        region: &'a DockRegion,
        ctx: &'a panels::PanelContext,
    ) -> Element<'a, DockMessage> }
```

## Type Parameters

- `'a`

## Source
Lines 254–342 in `crates/oxide-app/src/dock/view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/dock/view.md) |
| calls | [svg_icon](/crates/oxide-app/src/dock/view/svg_icon.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [estimate_text_width](/crates/oxide-app/src/dock/view/estimate_text_width.md) |
| calls | [rail_tab](/crates/oxide-app/src/styles/rail_tab.md) |
| calls | [collapsed_rail](/crates/oxide-app/src/styles/collapsed_rail.md) |
