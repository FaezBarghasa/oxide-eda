---
okf_version: "0.2"
type: Class
title: NetAntennaRule
description: Net Antenna / Dangling stub rule preventing dead-end open traces that radiate EMI.
resource: crates/oxide-rules/src/rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:26:29Z"
concept_id: crates/oxide-rules/src/rules/NetAntennaRule
language: rust
---

# NetAntennaRule

Net Antenna / Dangling stub rule preventing dead-end open traces that radiate EMI.

## Signature

```rust
pub struct NetAntennaRule
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Net Antenna / Dangling stub rule preventing dead-end open traces that radiate EMI.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `scope`
- `max_stub_length`

## Source
Lines 141–145 in `crates/oxide-rules/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-rules/src/rules.md) |
