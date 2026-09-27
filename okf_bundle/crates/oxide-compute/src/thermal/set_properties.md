---
okf_version: "0.2"
type: Function
title: set_properties
resource: crates/oxide-compute/src/thermal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/thermal/set_properties
language: rust
---

# set_properties

## Signature

```rust
impl ThermalSimulator { pub fn set_properties(&mut self, ambient_temp: f32, dt: f32, copper_k: f32, fr4_k: f32) }
```

## Visibility

- `pub`

## Source
Lines 57–62 in `crates/oxide-compute/src/thermal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal](/crates/oxide-compute/src/thermal.md) |
