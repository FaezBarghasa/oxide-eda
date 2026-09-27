---
okf_version: "0.2"
type: Class
title: PushAndShoveEngine
description: "Cascade push-and-shove physics engine: recursively resolves multi-obstacle collisions"
resource: crates/oxide-router/src/interactive/conflict.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:06:43Z"
concept_id: crates/oxide-router/src/interactive/conflict/PushAndShoveEngine
language: rust
---

# PushAndShoveEngine

Cascade push-and-shove physics engine: recursively resolves multi-obstacle collisions

## Signature

```rust
pub struct PushAndShoveEngine
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Cascade push-and-shove physics engine: recursively resolves multi-obstacle collisions
while maintaining strict clearance and minimum displacement energy.
[derive(Debug, Clone)]

## Methods

- `max_cascade_depth`
- `repulsive_spring_constant`

## Source
Lines 52–55 in `crates/oxide-router/src/interactive/conflict.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [conflict](/crates/oxide-router/src/interactive/conflict.md) |
