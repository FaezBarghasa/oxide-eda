---
okf_version: "0.2"
type: Function
title: from_pin_name
description: Infers electrical pin type from pin name / token heuristics.
resource: crates/oxide-library/src/harvester/types.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:15Z"
concept_id: crates/oxide-library/src/harvester/types/from_pin_name
language: rust
---

# from_pin_name

Infers electrical pin type from pin name / token heuristics.

## Signature

```rust
impl ElectricalPinType { pub fn from_pin_name(name: &str) -> Self }
```

## Visibility

- `pub`

## Docstring

Infers electrical pin type from pin name / token heuristics.

## Source
Lines 78–150 in `crates/oxide-library/src/harvester/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-library/src/harvester/types.md) |
