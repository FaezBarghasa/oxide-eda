---
okf_version: "0.2"
type: Function
title: standard_default
description: Standard baseline rules for typical 2-layer or 4-layer PCB fabrication
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/standard_default_1
language: rust
---

# standard_default

Standard baseline rules for typical 2-layer or 4-layer PCB fabrication

## Signature

```rust
pub fn standard_default() -> Self
```

## Visibility

- `pub`

## Docstring

Standard baseline rules for typical 2-layer or 4-layer PCB fabrication
(0.15mm min trace/space, 0.3mm drill / 0.6mm via pad, 50µm mask expansion, 100µm mask sliver).

## Source
Lines 36–77 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
