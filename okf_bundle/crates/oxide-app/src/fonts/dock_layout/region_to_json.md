---
okf_version: "0.2"
type: Function
title: region_to_json
resource: crates/oxide-app/src/fonts/dock_layout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:28:28Z"
concept_id: crates/oxide-app/src/fonts/dock_layout/region_to_json
language: rust
---

# region_to_json

## Signature

```rust
fn region_to_json(
        dock: &crate::dock::DockArea,
        pos: crate::dock::PanelPosition,
    ) -> serde_json::Value
```

## Source
Lines 8–21 in `crates/oxide-app/src/fonts/dock_layout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dock_layout](/crates/oxide-app/src/fonts/dock_layout.md) |
| calls | [panel_kind_key](/crates/oxide-app/src/fonts/dock_layout/panel_kind_key.md) |
