---
okf_version: "0.2"
type: Class
title: FilletTargets
description: "Solder fillet goals (Toe, Heel, Side) and Courtyard excess in millimeters."
resource: crates/oxide-bake/src/ipc7351/calculator.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:10:00Z"
concept_id: crates/oxide-bake/src/ipc7351/calculator/FilletTargets
language: rust
---

# FilletTargets

Solder fillet goals (Toe, Heel, Side) and Courtyard excess in millimeters.

## Signature

```rust
pub struct FilletTargets
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq)`

## Visibility

- `pub`

## Docstring

Solder fillet goals (Toe, Heel, Side) and Courtyard excess in millimeters.
[derive(Debug, Clone, Copy, PartialEq)]

## Methods

- `toe_jt`
- `heel_jh`
- `side_js`
- `courtyard_excess`

## Source
Lines 18–23 in `crates/oxide-bake/src/ipc7351/calculator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [calculator](/crates/oxide-bake/src/ipc7351/calculator.md) |
