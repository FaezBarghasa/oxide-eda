---
okf_version: "0.2"
type: Class
title: LayerDefinition
description: Single layer specification within a PCB stackup.
resource: crates/oxide-physics/src/stackup.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/stackup/LayerDefinition
language: rust
---

# LayerDefinition

Single layer specification within a PCB stackup.

## Signature

```rust
pub struct LayerDefinition
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Single layer specification within a PCB stackup.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `name`
- `layer_type`
- `material`
- `thickness`
- `copper_weight`

## Source
Lines 72–84 in `crates/oxide-physics/src/stackup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stackup](/crates/oxide-physics/src/stackup.md) |
