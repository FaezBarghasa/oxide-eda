---
okf_version: "0.2"
type: Function
title: set_pending_power_port
resource: crates/oxide-app/src/app/handlers/active_bar/placement_presets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/active_bar/placement_presets/set_pending_power_port
language: rust
---

# set_pending_power_port

## Signature

```rust
impl Oxide { fn set_pending_power_port(&mut self, net_name: &str, lib_id: &str) -> Task<Message> }
```

## Source
Lines 260–302 in `crates/oxide-app/src/app/handlers/active_bar/placement_presets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement_presets](/crates/oxide-app/src/app/handlers/active_bar/placement_presets.md) |
| calls | [Tool](/crates/oxide-app/src/app/documents/Tool.md) |
