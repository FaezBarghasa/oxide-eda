---
okf_version: "0.2"
type: Class
title: Polygon
description: "Closed polygon — points in mm. Used for courtyards, custom pads, etc."
resource: crates/oxide-library/src/primitive/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/footprint/pad/Polygon
language: rust
---

# Polygon

Closed polygon — points in mm. Used for courtyards, custom pads, etc.

## Signature

```rust
pub struct Polygon
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Closed polygon — points in mm. Used for courtyards, custom pads, etc.
[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]

## Methods

- `points`

## Source
Lines 108–110 in `crates/oxide-library/src/primitive/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-library/src/primitive/footprint/pad.md) |
| called_by | [extract_obstacles](/crates/oxide-router/src/topology/mod/extract_obstacles.md) |
