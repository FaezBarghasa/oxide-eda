---
okf_version: "0.2"
type: Function
title: standard_soic
description: Creates default standard SOIC dimensions for standard pin count.
resource: crates/oxide-library/src/harvester/types.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:15Z"
concept_id: crates/oxide-library/src/harvester/types/standard_soic
language: rust
---

# standard_soic

Creates default standard SOIC dimensions for standard pin count.

## Signature

```rust
impl PackageDimensions { pub fn standard_soic(pin_count: usize) -> Self }
```

## Visibility

- `pub`

## Docstring

Creates default standard SOIC dimensions for standard pin count.

## Source
Lines 204–219 in `crates/oxide-library/src/harvester/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-library/src/harvester/types.md) |
