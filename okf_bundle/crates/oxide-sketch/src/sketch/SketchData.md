---
okf_version: "0.2"
type: Class
title: SketchData
description: Top-level container for a footprint sketch. Persisted as part of
resource: crates/oxide-sketch/src/sketch.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/sketch/SketchData
language: rust
---

# SketchData

Top-level container for a footprint sketch. Persisted as part of

## Signature

```rust
pub struct SketchData
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Top-level container for a footprint sketch. Persisted as part of
a `Footprint`'s schema (v2+).
[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]

## Methods

- `planes`
- `entities`
- `constraints`
- `arrays`
- `parameters`

## Source
Lines 12–23 in `crates/oxide-sketch/src/sketch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch](/crates/oxide-sketch/src/sketch.md) |
