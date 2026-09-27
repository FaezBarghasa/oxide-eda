---
okf_version: "0.2"
type: Function
title: take_run
description: "Split off the leading run of same-kind bytes (all digits, or all non-digits)."
resource: crates/oxide-types/src/designator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/designator/take_run
language: rust
---

# take_run

Split off the leading run of same-kind bytes (all digits, or all non-digits).

## Signature

```rust
fn take_run(bytes: &[u8]) -> (&[u8], &[u8])
```

## Docstring

Split off the leading run of same-kind bytes (all digits, or all non-digits).

The input must be non-empty; callers check that before recursing.

## Source
Lines 14–21 in `crates/oxide-types/src/designator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [designator](/crates/oxide-types/src/designator.md) |
| called_by | [compare_references](/crates/oxide-types/src/designator/compare_references.md) |
