---
okf_version: "0.2"
type: Function
title: component_row_json_roundtrip
description: "`ComponentRow` round-trips through JSON without losing any"
resource: crates/oxide-library/src/component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/component/component_row_json_roundtrip
language: rust
---

# component_row_json_roundtrip

`ComponentRow` round-trips through JSON without losing any

## Signature

```rust
fn component_row_json_roundtrip()
```

## Decorators

- `test`

## Docstring

`ComponentRow` round-trips through JSON without losing any
fields. Foundational test for the rest of the row-tier work.
[test]

## Source
Lines 229–257 in `crates/oxide-library/src/component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component](/crates/oxide-library/src/component.md) |
| calls | [ComponentClass](/crates/oxide-library/src/identity/ComponentClass.md) |
