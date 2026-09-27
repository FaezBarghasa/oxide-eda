---
okf_version: "0.2"
type: Class
title: Probe
description: Simulation probe target.
resource: crates/oxide-types/src/sim.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:10:20Z"
concept_id: crates/oxide-types/src/sim/Probe
language: rust
---

# Probe

Simulation probe target.

## Signature

```rust
pub enum Probe
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(tag = "type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Simulation probe target.
[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(tag = "type", rename_all = "snake_case")]

## Methods

- `net_name`
- `net_id`
- `component_ref`
- `pin`
- `pos_net`
- `neg_net`

## Source
Lines 78–92 in `crates/oxide-types/src/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-types/src/sim.md) |
