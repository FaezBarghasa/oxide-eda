---
okf_version: "0.2"
type: Function
title: compare_text_runs
description: "Compare two text runs case-insensitively, so `r1` sits with `R1` rather"
resource: crates/oxide-types/src/designator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/designator/compare_text_runs
language: rust
---

# compare_text_runs

Compare two text runs case-insensitively, so `r1` sits with `R1` rather

## Signature

```rust
fn compare_text_runs(a: &[u8], b: &[u8]) -> Ordering
```

## Docstring

Compare two text runs case-insensitively, so `r1` sits with `R1` rather
than after every upper-case designator.

## Source
Lines 45–49 in `crates/oxide-types/src/designator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [designator](/crates/oxide-types/src/designator.md) |
| called_by | [compare_references](/crates/oxide-types/src/designator/compare_references.md) |
