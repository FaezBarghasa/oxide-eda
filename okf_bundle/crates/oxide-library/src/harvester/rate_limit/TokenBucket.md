---
okf_version: "0.2"
type: Class
title: TokenBucket
description: Token bucket rate limiter for distributor APIs.
resource: crates/oxide-library/src/harvester/rate_limit.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:41Z"
concept_id: crates/oxide-library/src/harvester/rate_limit/TokenBucket
language: rust
---

# TokenBucket

Token bucket rate limiter for distributor APIs.

## Signature

```rust
pub struct TokenBucket
```

## Decorators

- `derive(Debug)`

## Visibility

- `pub`

## Docstring

Token bucket rate limiter for distributor APIs.
[derive(Debug)]

## Methods

- `capacity`
- `refill_rate_per_sec`
- `tokens`
- `last_refill`

## Source
Lines 40–45 in `crates/oxide-library/src/harvester/rate_limit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rate_limit](/crates/oxide-library/src/harvester/rate_limit.md) |
