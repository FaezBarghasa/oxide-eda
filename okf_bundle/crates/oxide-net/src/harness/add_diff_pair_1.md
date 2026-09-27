---
okf_version: "0.2"
type: Function
title: add_diff_pair
description: Adds a differential pair to the harness.
resource: crates/oxide-net/src/harness.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:32:38Z"
concept_id: crates/oxide-net/src/harness/add_diff_pair_1
language: rust
---

# add_diff_pair

Adds a differential pair to the harness.

## Signature

```rust
pub fn add_diff_pair(&mut self, pair_name: &str, pos_net: &str, neg_net: &str)
```

## Visibility

- `pub`

## Docstring

Adds a differential pair to the harness.

## Source
Lines 58–64 in `crates/oxide-net/src/harness.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [harness](/crates/oxide-net/src/harness.md) |
