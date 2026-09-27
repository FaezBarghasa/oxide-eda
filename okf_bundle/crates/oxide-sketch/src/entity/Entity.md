---
okf_version: "0.2"
type: Class
title: Entity
description: "[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]"
resource: crates/oxide-sketch/src/entity.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/entity/Entity
language: rust
---

# Entity

[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Signature

```rust
pub struct Entity
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `id`
- `plane`
- `construction`
- `centerline`
- `kind`
- `pad`
- `silk`
- `courtyard`
- `mask_opening`
- `mask_exclude`
- `paste_aperture`
- `pour`
- `keepout`
- `board_cutout`
- `v_score`

## Source
Lines 11–56 in `crates/oxide-sketch/src/entity.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entity](/crates/oxide-sketch/src/entity.md) |
