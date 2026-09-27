---
okf_version: "0.2"
type: Class
title: LayerId
description: PCB layer identifier — minimal subset surfaced by the library layer.
resource: crates/oxide-library/src/primitive/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/footprint/pad/LayerId
language: rust
---

# LayerId

PCB layer identifier — minimal subset surfaced by the library layer.

## Signature

```rust
pub struct LayerId
```

## Decorators

- `derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(transparent)`

## Visibility

- `pub`

## Docstring

PCB layer identifier — minimal subset surfaced by the library layer.

The PCB editor (oxide-types::LayerId) carries the full Altium taxonomy.
This crate only needs to express which copper / mask / paste layers a pad
participates in; we keep a string-typed wrapper rather than importing
oxide-types here so this crate stays leaf-level.
[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(transparent)]

## Source
Lines 13–13 in `crates/oxide-library/src/primitive/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-library/src/primitive/footprint/pad.md) |
