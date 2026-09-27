---
okf_version: "0.2"
type: Class
title: Symbol
description: "Reusable schematic primitive. Bound by a `Component::symbol_ref`."
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/Symbol
language: rust
---

# Symbol

Reusable schematic primitive. Bound by a `Component::symbol_ref`.

## Signature

```rust
pub struct Symbol
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Reusable schematic primitive. Bound by a `Component::symbol_ref`.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `uuid`
- `name`
- `anchor`
- `pins`
- `graphics`
- `schematic_params`
- `designator`
- `comment`
- `description`
- `component_type`
- `mirrored`
- `local_fill_color`
- `local_line_color`
- `local_pin_color`
- `version`
- `released`
- `part_count`
- `created`
- `updated`

## Source
Lines 314–386 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
