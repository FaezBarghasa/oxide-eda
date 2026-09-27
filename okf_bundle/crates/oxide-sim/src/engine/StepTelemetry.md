---
okf_version: "0.2"
type: Class
title: StepTelemetry
description: "[repr(C)]"
resource: crates/oxide-sim/src/engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:33:35Z"
concept_id: crates/oxide-sim/src/engine/StepTelemetry
language: rust
---

# StepTelemetry

[repr(C)]

## Signature

```rust
pub struct StepTelemetry
```

## Decorators

- `repr(C)`
- `derive(Debug, Clone, Copy, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[repr(C)]
[derive(Debug, Clone, Copy, Serialize, Deserialize)]

## Methods

- `achieved_dt`
- `iterations`
- `lte_error`
- `convergence_stage`

## Source
Lines 44–49 in `crates/oxide-sim/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-sim/src/engine.md) |
