---
okf_version: "0.2"
type: Class
title: SchDrawing
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/oxide-types/src/schematic/sheet.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-types/src/schematic/sheet/SchDrawing
language: rust
---

# SchDrawing

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub enum SchDrawing
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`
- `serde(tag = "type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]
[serde(tag = "type", rename_all = "snake_case")]

## Methods

- `uuid`
- `start`
- `end`
- `width`
- `stroke_color`
- `uuid`
- `start`
- `end`
- `width`
- `fill`
- `stroke_color`
- `uuid`
- `center`
- `radius`
- `width`
- `fill`
- `stroke_color`
- `uuid`
- `start`
- `mid`
- `end`
- `width`
- `fill`
- `stroke_color`
- `uuid`
- `points`
- `width`
- `fill`
- `stroke_color`

## Source
Lines 157–211 in `crates/oxide-types/src/schematic/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-types/src/schematic/sheet.md) |
