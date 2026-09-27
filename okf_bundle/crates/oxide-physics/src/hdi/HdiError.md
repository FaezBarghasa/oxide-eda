---
okf_version: "0.2"
type: Class
title: HdiError
description: Violations resulting from invalid via geometry or impossible layer spans.
resource: crates/oxide-physics/src/hdi.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/hdi/HdiError
language: rust
---

# HdiError

Violations resulting from invalid via geometry or impossible layer spans.

## Signature

```rust
pub enum HdiError
```

## Decorators

- `derive(Debug, Clone, PartialEq, Error)`

## Visibility

- `pub`

## Docstring

Violations resulting from invalid via geometry or impossible layer spans.
[derive(Debug, Clone, PartialEq, Error)]

## Methods

- `start`
- `end`
- `total_layers`
- `start`
- `end`
- `start_type`
- `end_type`
- `start`
- `end`
- `start`
- `end`
- `outer_first`
- `outer_last`
- `start`
- `end`
- `outer_first`
- `outer_last`
- `depth_microns`
- `diameter_microns`
- `aspect_ratio`
- `span`

## Source
Lines 70–126 in `crates/oxide-physics/src/hdi.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hdi](/crates/oxide-physics/src/hdi.md) |
