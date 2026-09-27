---
okf_version: "0.2"
type: Function
title: where_used
description: "Find every site where `row_id` is used. Returned order is"
resource: crates/oxide-library/src/where_used.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/where_used/where_used_1
language: rust
---

# where_used

Find every site where `row_id` is used. Returned order is

## Signature

```rust
pub fn where_used(&self, row_id: RowId) -> Vec<UseSite>
```

## Visibility

- `pub`

## Docstring

Find every site where `row_id` is used. Returned order is
unspecified — callers should sort if they need determinism.

## Source
Lines 173–190 in `crates/oxide-library/src/where_used.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [where_used](/crates/oxide-library/src/where_used.md) |
