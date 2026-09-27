---
okf_version: "0.2"
type: Function
title: gpu_draws_buckets_the_pcb_cpu_path_omits
description: The GPU path composites arc and text buckets the PCB CPU path never
resource: crates/oxide-gfx/src/scene/order.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/scene/order/gpu_draws_buckets_the_pcb_cpu_path_omits
language: rust
---

# gpu_draws_buckets_the_pcb_cpu_path_omits

The GPU path composites arc and text buckets the PCB CPU path never

## Signature

```rust
fn gpu_draws_buckets_the_pcb_cpu_path_omits()
```

## Decorators

- `test`

## Docstring

The GPU path composites arc and text buckets the PCB CPU path never
emits. Harmless today (PCB scenes carry neither) but a real parity axis:
if the PCB scene ever grows arcs or text, the CPU renderer must gain the
buckets too.
[test]

## Source
Lines 304–309 in `crates/oxide-gfx/src/scene/order.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [order](/crates/oxide-gfx/src/scene/order.md) |
