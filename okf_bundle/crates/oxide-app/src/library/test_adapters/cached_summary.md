---
okf_version: "0.2"
type: Function
title: cached_summary
description: "A `PrimitiveSummary` standing in for an entry already on disk and"
resource: crates/oxide-app/src/library/test_adapters.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/test_adapters/cached_summary
language: rust
---

# cached_summary

A `PrimitiveSummary` standing in for an entry already on disk and

## Signature

```rust
pub(crate) fn cached_summary(name: &str) -> PrimitiveSummary
```

## Visibility

- `pub(crate)`

## Docstring

A `PrimitiveSummary` standing in for an entry already on disk and
already cached — the thing a failed listing used to erase.

## Source
Lines 69–76 in `crates/oxide-app/src/library/test_adapters.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [test_adapters](/crates/oxide-app/src/library/test_adapters.md) |
