---
okf_version: "0.2"
type: Function
title: mid_too_close_to_endpoint_on_a_long_line_is_rejected
description: "[test]"
resource: crates/oxide-sketch/src/split/tests/errors.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/tests/errors/mid_too_close_to_endpoint_on_a_long_line_is_rejected
language: rust
---

# mid_too_close_to_endpoint_on_a_long_line_is_rejected

[test]

## Signature

```rust
fn mid_too_close_to_endpoint_on_a_long_line_is_rejected()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 27–36 in `crates/oxide-sketch/src/split/tests/errors.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [errors](/crates/oxide-sketch/src/split/tests/errors.md) |
| calls | [line_sketch](/crates/oxide-sketch/src/split/tests/mod/line_sketch.md) |
| calls | [split_line](/crates/oxide-sketch/src/split/mod/split_line.md) |
