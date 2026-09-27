---
okf_version: "0.2"
type: Function
title: bench
description: "Run one bench: time `f` over `iters` runs and return the mean"
resource: crates/oxide-sketch/examples/bench_linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-sketch/examples/bench_linalg/bench
language: rust
---

# bench

Run one bench: time `f` over `iters` runs and return the mean

## Signature

```rust
fn bench(iters: usize, mut f: F) -> f64
```

## Type Parameters

- `F: FnMut()`

## Docstring

Run one bench: time `f` over `iters` runs and return the mean
duration in nanoseconds.

## Source
Lines 70–79 in `crates/oxide-sketch/examples/bench_linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bench_linalg](/crates/oxide-sketch/examples/bench_linalg.md) |
| called_by | [main](/crates/oxide-sketch/examples/bench_linalg/main.md) |
