---
okf_version: "0.2"
type: Class
title: ArraySummary
description: v0.23 — Array (Pattern) properties surfaced on the Properties panel
resource: crates/oxide-app/src/panels/footprint_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_context/ArraySummary
language: rust
---

# ArraySummary

v0.23 — Array (Pattern) properties surfaced on the Properties panel

## Signature

```rust
pub struct ArraySummary
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

v0.23 — Array (Pattern) properties surfaced on the Properties panel
when the selected sketch entity is the source of an
[`oxide_sketch::array::Array`]. The handler resolves the array by
`array_id`, mutates the matching field, then runs solve+bake.
[derive(Debug, Clone)]

## Methods

- `array_id`
- `kind`
- `numbering`
- `repicking_polar_center`
- `bga_config`

## Source
Lines 266–278 in `crates/oxide-app/src/panels/footprint_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_context](/crates/oxide-app/src/panels/footprint_context.md) |
