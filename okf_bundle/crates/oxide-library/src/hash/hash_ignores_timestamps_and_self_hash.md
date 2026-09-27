---
okf_version: "0.2"
type: Function
title: hash_ignores_timestamps_and_self_hash
description: "Bookkeeping fields (`created`, `updated`, `content_hash`) MUST NOT"
resource: crates/oxide-library/src/hash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/hash/hash_ignores_timestamps_and_self_hash
language: rust
---

# hash_ignores_timestamps_and_self_hash

Bookkeeping fields (`created`, `updated`, `content_hash`) MUST NOT

## Signature

```rust
fn hash_ignores_timestamps_and_self_hash()
```

## Decorators

- `test`

## Docstring

Bookkeeping fields (`created`, `updated`, `content_hash`) MUST NOT
affect the content hash — that's the whole point of distinguishing
technical content from save metadata.
[test]

## Source
Lines 206–216 in `crates/oxide-library/src/hash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hash](/crates/oxide-library/src/hash.md) |
| calls | [fixture_row](/crates/oxide-library/src/hash/fixture_row.md) |
