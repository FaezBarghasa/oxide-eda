---
okf_version: "0.2"
type: Function
title: add_net
description: Adds a single-ended net to the harness.
resource: crates/oxide-net/src/harness.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:32:38Z"
concept_id: crates/oxide-net/src/harness/add_net_1
language: rust
---

# add_net

Adds a single-ended net to the harness.

## Signature

```rust
pub fn add_net(&mut self, net_name: &str)
```

## Visibility

- `pub`

## Docstring

Adds a single-ended net to the harness.

## Source
Lines 44–46 in `crates/oxide-net/src/harness.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [harness](/crates/oxide-net/src/harness.md) |
| calls | [Net](/crates/oxide-types/src/net/Net.md) |
