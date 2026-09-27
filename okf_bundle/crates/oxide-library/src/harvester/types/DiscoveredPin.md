---
okf_version: "0.2"
type: Class
title: DiscoveredPin
description: Discovered pin metadata extracted from unstructured datasheets or structured APIs.
resource: crates/oxide-library/src/harvester/types.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:15Z"
concept_id: crates/oxide-library/src/harvester/types/DiscoveredPin
language: rust
---

# DiscoveredPin

Discovered pin metadata extracted from unstructured datasheets or structured APIs.

## Signature

```rust
pub struct DiscoveredPin
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Discovered pin metadata extracted from unstructured datasheets or structured APIs.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `number`
- `name`
- `electrical_type`
- `alternate_functions`
- `description`
- `io_bank`

## Source
Lines 155–165 in `crates/oxide-library/src/harvester/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-library/src/harvester/types.md) |
