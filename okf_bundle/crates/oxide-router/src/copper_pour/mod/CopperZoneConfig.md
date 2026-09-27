---
okf_version: "0.2"
type: Class
title: CopperZoneConfig
description: Dynamic copper zone definition with priority and thermal relief configuration.
resource: crates/oxide-router/src/copper_pour/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:53:15Z"
concept_id: crates/oxide-router/src/copper_pour/mod/CopperZoneConfig
language: rust
---

# CopperZoneConfig

Dynamic copper zone definition with priority and thermal relief configuration.

## Signature

```rust
pub struct CopperZoneConfig
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Dynamic copper zone definition with priority and thermal relief configuration.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `net_id`
- `layer`
- `priority`
- `clearance`
- `min_island_area_sq_microns`
- `thermal_relief`
- `thermal_spoke_width`
- `thermal_gap`

## Source
Lines 25–34 in `crates/oxide-router/src/copper_pour/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [copper_pour](/crates/oxide-router/src/copper_pour/mod.md) |
