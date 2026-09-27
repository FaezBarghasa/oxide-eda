---
okf_version: "0.2"
type: Function
title: loaded_project_data_round_trips_via_write_then_parse
description: "[test]"
resource: crates/oxide-app/tests/regression/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:39Z"
concept_id: crates/oxide-app/tests/regression/project/loaded_project_data_round_trips_via_write_then_parse
language: rust
---

# loaded_project_data_round_trips_via_write_then_parse

[test]

## Signature

```rust
fn loaded_project_data_round_trips_via_write_then_parse()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 602–626 in `crates/oxide-app/tests/regression/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/tests/regression/project.md) |
| calls | [write_project](/crates/oxide-app/tests/measure_library_open/write_project.md) |
| calls | [parse_project](/crates/oxide-types/src/project/parse_project.md) |
