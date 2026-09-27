---
okf_version: "0.2"
type: Function
title: view_dock_panel_h
resource: crates/oxide-app/src/app/view/chrome.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/chrome/view_dock_panel_h_1
language: rust
---

# view_dock_panel_h

## Signature

```rust
pub(super) fn view_dock_panel_h(
        &self,
        pos: PanelPosition,
        has_panels: bool,
        collapsed: bool,
        size: f32,
    ) -> Element<'_, Message>
```

## Visibility

- `pub(super)`

## Source
Lines 546–572 in `crates/oxide-app/src/app/view/chrome.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chrome](/crates/oxide-app/src/app/view/chrome.md) |
| calls | [panel_region](/crates/oxide-app/src/styles/panel_region.md) |
