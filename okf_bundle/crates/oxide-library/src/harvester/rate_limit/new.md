---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-library/src/harvester/rate_limit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:41Z"
concept_id: crates/oxide-library/src/harvester/rate_limit/new
language: rust
---

# new

## Signature

```rust
impl TokenBucket { pub fn new(capacity: f64, refill_rate_per_sec: f64) -> Self }
```

## Visibility

- `pub`

## Source
Lines 48–55 in `crates/oxide-library/src/harvester/rate_limit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rate_limit](/crates/oxide-library/src/harvester/rate_limit.md) |
