---
okf_version: "0.2"
type: Class
title: RoutingPath
description: Complete routing path for a routed net.
resource: crates/oxide-router/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:03:56Z"
concept_id: crates/oxide-router/src/lib/RoutingPath
language: rust
---

# RoutingPath

Complete routing path for a routed net.

## Signature

```rust
pub struct RoutingPath
```

## Decorators

- `derive(Debug, Clone, PartialEq, Default)`

## Visibility

- `pub`

## Docstring

Complete routing path for a routed net.
[derive(Debug, Clone, PartialEq, Default)]

## Methods

- `net_id`
- `segments`
- `vias`
- `total_length`
- `layer_transitions`

## Source
Lines 97–103 in `crates/oxide-router/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-router/src/lib.md) |
