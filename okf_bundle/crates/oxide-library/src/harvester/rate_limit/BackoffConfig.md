---
okf_version: "0.2"
type: Class
title: BackoffConfig
description: Exponential backoff calculator with randomized jitter for distributor scraping and crawling.
resource: crates/oxide-library/src/harvester/rate_limit.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:41Z"
concept_id: crates/oxide-library/src/harvester/rate_limit/BackoffConfig
language: rust
---

# BackoffConfig

Exponential backoff calculator with randomized jitter for distributor scraping and crawling.

## Signature

```rust
pub struct BackoffConfig
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Exponential backoff calculator with randomized jitter for distributor scraping and crawling.
[derive(Debug, Clone)]

## Methods

- `base_ms`
- `max_ms`
- `jitter_factor`

## Source
Lines 5–9 in `crates/oxide-library/src/harvester/rate_limit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rate_limit](/crates/oxide-library/src/harvester/rate_limit.md) |
