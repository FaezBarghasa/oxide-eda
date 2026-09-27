---
okf_version: "0.2"
type: Function
title: hotkey_drop_via_and_switch_layer
description: "Hotkey '*': Drop a via at current position and switch active routing layer"
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/hotkey_drop_via_and_switch_layer_1
language: rust
---

# hotkey_drop_via_and_switch_layer

Hotkey '*': Drop a via at current position and switch active routing layer

## Signature

```rust
pub fn hotkey_drop_via_and_switch_layer(
        &mut self,
        target_layer: LayerId,
        diameter_microns: Microns,
        drill_microns: Microns,
    ) -> Option<crate::ViaPlacement>
```

## Visibility

- `pub`

## Docstring

Hotkey '*': Drop a via at current position and switch active routing layer

## Source
Lines 135–153 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
