---
okf_version: "0.2"
type: Class
title: Zone
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/oxide-types/src/pcb.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:21:51Z"
concept_id: crates/oxide-types/src/pcb/Zone
language: rust
---

# Zone

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct Zone
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `uuid`
- `net`
- `net_name`
- `layer`
- `outline`
- `priority`
- `fill_type`
- `thermal_relief`
- `thermal_gap`
- `thermal_width`
- `clearance`
- `min_thickness`

## Source
Lines 302–326 in `crates/oxide-types/src/pcb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb](/crates/oxide-types/src/pcb.md) |
