---
okf_version: "0.2"
type: Class
title: HarnessElement
description: Signal element contained within a structured signal harness.
resource: crates/oxide-net/src/harness.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:32:38Z"
concept_id: crates/oxide-net/src/harness/HarnessElement
language: rust
---

# HarnessElement

Signal element contained within a structured signal harness.

## Signature

```rust
pub enum HarnessElement
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Signal element contained within a structured signal harness.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `name`
- `msb`
- `lsb`
- `pair_name`
- `pos_net`
- `neg_net`

## Source
Lines 11–26 in `crates/oxide-net/src/harness.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [harness](/crates/oxide-net/src/harness.md) |
