---
okf_version: "0.2"
type: Function
title: calculate_delay
description: "Calculates backoff duration for a given retry attempt:"
resource: crates/oxide-library/src/harvester/rate_limit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:41Z"
concept_id: crates/oxide-library/src/harvester/rate_limit/calculate_delay
language: rust
---

# calculate_delay

Calculates backoff duration for a given retry attempt:

## Signature

```rust
impl BackoffConfig { pub fn calculate_delay(&self, retry: u32) -> Duration }
```

## Visibility

- `pub`

## Docstring

Calculates backoff duration for a given retry attempt:
t_backoff = min(t_max, t_base * 2^retry) ± jitter

## Source
Lines 24–35 in `crates/oxide-library/src/harvester/rate_limit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rate_limit](/crates/oxide-library/src/harvester/rate_limit.md) |
