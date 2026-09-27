---
okf_version: "0.2"
type: Class
title: RouteSegment
description: A single segment in a routed path.
resource: crates/oxide-router/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:03:56Z"
concept_id: crates/oxide-router/src/lib/RouteSegment
language: rust
---

# RouteSegment

A single segment in a routed path.

## Signature

```rust
pub struct RouteSegment
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

A single segment in a routed path.
[derive(Debug, Clone, PartialEq)]

## Methods

- `start_point`
- `end_point`
- `width`
- `layer`
- `net_id`
- `segment_type`

## Source
Lines 107–114 in `crates/oxide-router/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-router/src/lib.md) |
