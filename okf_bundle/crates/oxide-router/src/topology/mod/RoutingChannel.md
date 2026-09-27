---
okf_version: "0.2"
type: Class
title: RoutingChannel
description: Routing channel between obstacles
resource: crates/oxide-router/src/topology/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/mod/RoutingChannel
language: rust
---

# RoutingChannel

Routing channel between obstacles

## Signature

```rust
pub struct RoutingChannel
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Routing channel between obstacles
[derive(Debug, Clone, PartialEq)]

## Methods

- `id`
- `start_obstacle`
- `end_obstacle`
- `width`
- `capacity`
- `used_tracks`

## Source
Lines 109–116 in `crates/oxide-router/src/topology/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-router/src/topology/mod.md) |
