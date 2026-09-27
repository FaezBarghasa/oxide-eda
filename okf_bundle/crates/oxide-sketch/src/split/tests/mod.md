---
okf_version: "0.2"
type: Module
title: tests
description: "`split_line` tests, grouped by concern (kept under the ~800-line"
resource: crates/oxide-sketch/src/split/tests/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/tests/mod
language: rust
---

# tests

`split_line` tests, grouped by concern (kept under the ~800-line

## Docstring

`split_line` tests, grouped by concern (kept under the ~800-line
file cap — `oxide-domain` §5):
- `carry_over` — attribute / flag / constraint / array / pad
profile-seed carry-over onto the two replacement halves.
- `errors` — validation + the degenerate-input error taxonomy.
- `solver` — end-to-end solver acceptance (issue #360 blocker 3).

## Relationships

| Type | Target |
|------|--------|
| related | [line_sketch](/crates/oxide-sketch/src/split/tests/mod/line_sketch.md) |
| related | [line_endpoints](/crates/oxide-sketch/src/split/tests/mod/line_endpoints.md) |
