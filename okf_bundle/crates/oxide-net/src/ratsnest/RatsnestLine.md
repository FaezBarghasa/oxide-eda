---
okf_version: "0.2"
type: Class
title: RatsnestLine
description: An unrouted connection between two physical points on a PCB layout.
resource: crates/oxide-net/src/ratsnest.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:14:41Z"
concept_id: crates/oxide-net/src/ratsnest/RatsnestLine
language: rust
---

# RatsnestLine

An unrouted connection between two physical points on a PCB layout.

## Signature

```rust
pub struct RatsnestLine
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

An unrouted connection between two physical points on a PCB layout.
[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]

## Methods

- `net_number`
- `start`
- `end`
- `distance`

## Source
Lines 13–18 in `crates/oxide-net/src/ratsnest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ratsnest](/crates/oxide-net/src/ratsnest.md) |
