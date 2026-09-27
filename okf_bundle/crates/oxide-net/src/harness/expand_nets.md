---
okf_version: "0.2"
type: Function
title: expand_nets
description: Expands the entire harness into a flat list of constituent net names.
resource: crates/oxide-net/src/harness.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:32:38Z"
concept_id: crates/oxide-net/src/harness/expand_nets
language: rust
---

# expand_nets

Expands the entire harness into a flat list of constituent net names.

## Signature

```rust
impl SignalHarness { pub fn expand_nets(&self) -> Vec<String> }
```

## Visibility

- `pub`

## Docstring

Expands the entire harness into a flat list of constituent net names.

## Source
Lines 67–86 in `crates/oxide-net/src/harness.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [harness](/crates/oxide-net/src/harness.md) |
