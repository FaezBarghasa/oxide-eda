---
okf_version: "0.2"
type: Function
title: project_data_loads_without_libraries_field
description: "`.snxprj` files written before the `libraries` field landed"
resource: crates/oxide-types/src/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:17Z"
concept_id: crates/oxide-types/src/project/project_data_loads_without_libraries_field
language: rust
---

# project_data_loads_without_libraries_field

`.snxprj` files written before the `libraries` field landed

## Signature

```rust
fn project_data_loads_without_libraries_field()
```

## Decorators

- `test`

## Docstring

`.snxprj` files written before the `libraries` field landed
must round-trip cleanly with an empty list. Backwards-compat
is the load-bearing constraint here — we cannot break
existing project files.
[test]

## Source
Lines 253–263 in `crates/oxide-types/src/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-types/src/project.md) |
