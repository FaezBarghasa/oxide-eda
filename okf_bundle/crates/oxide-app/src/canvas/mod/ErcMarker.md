---
okf_version: "0.2"
type: Class
title: ErcMarker
description: Canvas-side projection of an ERC violation — just enough to draw
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/ErcMarker
language: rust
---

# ErcMarker

Canvas-side projection of an ERC violation — just enough to draw

## Signature

```rust
pub struct ErcMarker
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Canvas-side projection of an ERC violation — just enough to draw
its marker without pulling the full Violation type into the render
crate.
[derive(Debug, Clone)]

## Methods

- `x`
- `y`
- `severity`
- `primary_uuid`

## Source
Lines 155–162 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
