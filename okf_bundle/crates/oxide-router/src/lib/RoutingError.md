---
okf_version: "0.2"
type: Class
title: RoutingError
description: Routing error taxonomy.
resource: crates/oxide-router/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:03:56Z"
concept_id: crates/oxide-router/src/lib/RoutingError
language: rust
---

# RoutingError

Routing error taxonomy.

## Signature

```rust
pub enum RoutingError
```

## Decorators

- `derive(Debug, Clone, PartialEq, thiserror::Error)`

## Visibility

- `pub`

## Docstring

Routing error taxonomy.
[derive(Debug, Clone, PartialEq, thiserror::Error)]

## Methods

- `location`
- `actual`
- `required`
- `net`
- `layer`
- `net`
- `actual`
- `target`
- `region`
- `density`

## Source
Lines 74–93 in `crates/oxide-router/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-router/src/lib.md) |
