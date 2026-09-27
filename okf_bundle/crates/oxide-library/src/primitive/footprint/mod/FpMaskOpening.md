---
okf_version: "0.2"
type: Class
title: FpMaskOpening
description: Solder-mask opening (cutout) — copper without solder mask covering.
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/FpMaskOpening
language: rust
---

# FpMaskOpening

Solder-mask opening (cutout) — copper without solder mask covering.

## Signature

```rust
pub struct FpMaskOpening
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Solder-mask opening (cutout) — copper without solder mask covering.
Distinct from a pad's mask margin: this is a standalone profile for
e.g. an exposed copper region or a panel-level marker.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `boundary`
- `layer`

## Source
Lines 271–274 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
