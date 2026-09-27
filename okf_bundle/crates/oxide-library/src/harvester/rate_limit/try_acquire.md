---
okf_version: "0.2"
type: Function
title: try_acquire
resource: crates/oxide-library/src/harvester/rate_limit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:41Z"
concept_id: crates/oxide-library/src/harvester/rate_limit/try_acquire
language: rust
---

# try_acquire

## Signature

```rust
impl TokenBucket { pub fn try_acquire(&mut self, required_tokens: f64) -> bool }
```

## Visibility

- `pub`

## Source
Lines 57–65 in `crates/oxide-library/src/harvester/rate_limit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rate_limit](/crates/oxide-library/src/harvester/rate_limit.md) |
