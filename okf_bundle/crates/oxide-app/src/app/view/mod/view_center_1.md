---
okf_version: "0.2"
type: Function
title: view_center
resource: crates/oxide-app/src/app/view/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/mod/view_center_1
language: rust
---

# view_center

## Signature

```rust
fn view_center(&self, window_id: iced::window::Id) -> Element<'_, Message>
```

## Source
Lines 501–703 in `crates/oxide-app/src/app/view/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/app/view/mod.md) |
| calls | [panel_region](/crates/oxide-app/src/styles/panel_region.md) |
| calls | [view_symbol](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol.md) |
| calls | [view_footprint](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
