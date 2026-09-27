---
okf_version: "0.2"
type: Function
title: for_package
description: Selects fillet targets matching package class string.
resource: crates/oxide-bake/src/ipc7351/calculator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:10:00Z"
concept_id: crates/oxide-bake/src/ipc7351/calculator/for_package
language: rust
---

# for_package

Selects fillet targets matching package class string.

## Signature

```rust
impl FilletTargets { pub fn for_package(pkg_class: &str, density: DensityLevel) -> Self }
```

## Visibility

- `pub`

## Docstring

Selects fillet targets matching package class string.

## Source
Lines 99–108 in `crates/oxide-bake/src/ipc7351/calculator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [calculator](/crates/oxide-bake/src/ipc7351/calculator.md) |
