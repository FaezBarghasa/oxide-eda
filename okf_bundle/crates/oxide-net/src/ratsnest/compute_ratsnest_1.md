---
okf_version: "0.2"
type: Function
title: compute_ratsnest
description: Compute all unrouted ratsnest lines for a given PCB layout.
resource: crates/oxide-net/src/ratsnest.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:14:41Z"
concept_id: crates/oxide-net/src/ratsnest/compute_ratsnest_1
language: rust
---

# compute_ratsnest

Compute all unrouted ratsnest lines for a given PCB layout.

## Signature

```rust
pub fn compute_ratsnest(board: &PcbBoard) -> Vec<RatsnestLine>
```

## Visibility

- `pub`

## Docstring

Compute all unrouted ratsnest lines for a given PCB layout.

## Source
Lines 25–65 in `crates/oxide-net/src/ratsnest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ratsnest](/crates/oxide-net/src/ratsnest.md) |
