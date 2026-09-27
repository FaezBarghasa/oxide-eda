---
okf_version: "0.2"
type: Class
title: SimModel
description: "Reusable simulation model. Bound by `Component::sim_ref`."
resource: crates/oxide-library/src/primitive/sim.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:11:54Z"
concept_id: crates/oxide-library/src/primitive/sim/SimModel
language: rust
---

# SimModel

Reusable simulation model. Bound by `Component::sim_ref`.

## Signature

```rust
pub struct SimModel
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Reusable simulation model. Bound by `Component::sim_ref`.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `uuid`
- `name`
- `kind`
- `body`
- `default_node_map`
- `version`
- `released`
- `created`
- `updated`

## Source
Lines 34–54 in `crates/oxide-library/src/primitive/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-library/src/primitive/sim.md) |
