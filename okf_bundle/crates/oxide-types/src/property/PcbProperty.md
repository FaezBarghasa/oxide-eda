---
okf_version: "0.2"
type: Class
title: PcbProperty
description: Captures Standard footprint property metadata while preserving the legacy
resource: crates/oxide-types/src/property.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/property/PcbProperty
language: rust
---

# PcbProperty

Captures Standard footprint property metadata while preserving the legacy

## Signature

```rust
pub struct PcbProperty
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Captures Standard footprint property metadata while preserving the legacy
footprint `reference` and `value` compatibility fields.
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `key`
- `value`
- `position`
- `rotation`
- `layer`
- `font_size`
- `hidden`

## Source
Lines 30–45 in `crates/oxide-types/src/property.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [property](/crates/oxide-types/src/property.md) |
