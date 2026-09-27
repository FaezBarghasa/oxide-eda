---
okf_version: "0.2"
type: Class
title: SceneBucket
description: "A drawable bucket of a [`Scene`](crate::scene::Scene). Every variant maps"
resource: crates/oxide-gfx/src/scene/order.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/scene/order/SceneBucket
language: rust
---

# SceneBucket

A drawable bucket of a [`Scene`](crate::scene::Scene). Every variant maps

## Signature

```rust
pub enum SceneBucket
```

## Decorators

- `derive(Clone, Copy, PartialEq, Eq, Debug)`

## Visibility

- `pub`

## Docstring

A drawable bucket of a [`Scene`](crate::scene::Scene). Every variant maps
1:1 to a `Vec` field on the scene; the order of a slice of these is a draw
(z) order, back to front.
[derive(Clone, Copy, PartialEq, Eq, Debug)]

## Source
Lines 33–45 in `crates/oxide-gfx/src/scene/order.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [order](/crates/oxide-gfx/src/scene/order.md) |
