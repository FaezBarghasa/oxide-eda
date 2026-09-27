---
okf_version: "0.2"
type: Function
title: unbounded_field_keeps_negative_and_large_values
description: "The unbounded rotation field keeps every finite value, including"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/unbounded_field_keeps_negative_and_large_values
language: rust
---

# unbounded_field_keeps_negative_and_large_values

The unbounded rotation field keeps every finite value, including

## Signature

```rust
fn unbounded_field_keeps_negative_and_large_values()
```

## Decorators

- `test`

## Docstring

The unbounded rotation field keeps every finite value, including
negatives — it has no range filter and must not grow one here.
[test]

## Source
Lines 183–200 in `crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numeric_input](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.md) |
