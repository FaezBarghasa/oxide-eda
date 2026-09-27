---
okf_version: "0.2"
type: Class
title: FailingListingAdapter
description: A mounted library whose three primitive listings always fail.
resource: crates/oxide-app/src/library/test_adapters.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/test_adapters/FailingListingAdapter
language: rust
---

# FailingListingAdapter

A mounted library whose three primitive listings always fail.

## Signature

```rust
pub(crate) struct FailingListingAdapter
```

## Visibility

- `pub(crate)`

## Docstring

A mounted library whose three primitive listings always fail.

Models the reachable cases — a `symbols/` directory that cannot be
read, a library server returning 500 — as the `LibraryError` those
paths actually produce. `list_tables` succeeds with no tables so
`refresh_components` reaches the primitive listings.

## Methods

- `manifest`

## Source
Lines 19–21 in `crates/oxide-app/src/library/test_adapters.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [test_adapters](/crates/oxide-app/src/library/test_adapters.md) |
