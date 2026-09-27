---
okf_version: "0.2"
type: Function
title: bad_parse_keeps_buffer_dirty_and_skips_commit
description: "Bad parse path: an in-progress numeric input (\"12.5e\") leaves"
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/bad_parse_keeps_buffer_dirty_and_skips_commit
language: rust
---

# bad_parse_keeps_buffer_dirty_and_skips_commit

Bad parse path: an in-progress numeric input ("12.5e") leaves

## Signature

```rust
fn bad_parse_keeps_buffer_dirty_and_skips_commit()
```

## Decorators

- `test`

## Docstring

Bad parse path: an in-progress numeric input ("12.5e") leaves
the buffer dirty and skips the commit, so the parameter stays
at its previous value (or absent). Mirrors the contract for
`ParamCommitNumber` / `ParamCommitMeasurement` in the dispatcher.
[test]

## Source
Lines 650–672 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
