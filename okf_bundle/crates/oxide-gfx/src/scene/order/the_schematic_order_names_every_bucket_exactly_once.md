---
okf_version: "0.2"
type: Function
title: the_schematic_order_names_every_bucket_exactly_once
description: "The schematic replay is the only path that draws every bucket, so its"
resource: crates/oxide-gfx/src/scene/order.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/scene/order/the_schematic_order_names_every_bucket_exactly_once
language: rust
---

# the_schematic_order_names_every_bucket_exactly_once

The schematic replay is the only path that draws every bucket, so its

## Signature

```rust
fn the_schematic_order_names_every_bucket_exactly_once()
```

## Decorators

- `test`

## Docstring

The schematic replay is the only path that draws every bucket, so its
order is also the completeness check: add a `SceneBucket` variant and
this fails until someone decides where it belongs.
[test]

## Source
Lines 213–236 in `crates/oxide-gfx/src/scene/order.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [order](/crates/oxide-gfx/src/scene/order.md) |
