---
okf_version: "0.2"
type: Module
title: common
description: Shared test fixtures for the constraint-residual integration tests.
resource: crates/oxide-sketch/tests/common/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/common/mod
language: rust
---

# common

Shared test fixtures for the constraint-residual integration tests.

## Docstring

Shared test fixtures for the constraint-residual integration tests.

Test files (`constraints_*.rs`) compile as separate crates, but they
all share the same `oxide-sketch` API. Concrete sketch builders
live here so each test file can import them via
`mod common;` and avoid drifting fixtures.

## Relationships

| Type | Target |
|------|--------|
| related | [Sketch](/crates/oxide-sketch/tests/common/mod/Sketch.md) |
| related | [new](/crates/oxide-sketch/tests/common/mod/new.md) |
| related | [add_point](/crates/oxide-sketch/tests/common/mod/add_point.md) |
| related | [add_line](/crates/oxide-sketch/tests/common/mod/add_line.md) |
| related | [add_arc](/crates/oxide-sketch/tests/common/mod/add_arc.md) |
| related | [add_circle](/crates/oxide-sketch/tests/common/mod/add_circle.md) |
| related | [new](/crates/oxide-sketch/tests/common/mod/new.md) |
| related | [add_point](/crates/oxide-sketch/tests/common/mod/add_point.md) |
| related | [add_line](/crates/oxide-sketch/tests/common/mod/add_line.md) |
| related | [add_arc](/crates/oxide-sketch/tests/common/mod/add_arc.md) |
| related | [add_circle](/crates/oxide-sketch/tests/common/mod/add_circle.md) |
