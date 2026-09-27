---
okf_version: "0.2"
type: Function
title: measurement_parse_round_trip_via_buffer
description: "Measurement parse round-trip via the buffer pattern: a typed"
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/measurement_parse_round_trip_via_buffer
language: rust
---

# measurement_parse_round_trip_via_buffer

Measurement parse round-trip via the buffer pattern: a typed

## Signature

```rust
fn measurement_parse_round_trip_via_buffer()
```

## Decorators

- `test`

## Docstring

Measurement parse round-trip via the buffer pattern: a typed
string commits into a `ParamValue::Measurement` with the
template's unit.
[test]

## Source
Lines 616–643 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
