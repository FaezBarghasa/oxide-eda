---
okf_version: "0.2"
type: Class
title: MemorySegment
description: Memory segment extracted from firmware image.
resource: crates/oxide-mcu/src/firmware.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:44:25Z"
concept_id: crates/oxide-mcu/src/firmware/MemorySegment
language: rust
---

# MemorySegment

Memory segment extracted from firmware image.

## Signature

```rust
pub struct MemorySegment
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Memory segment extracted from firmware image.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `addr`
- `size`
- `name`

## Source
Lines 539–543 in `crates/oxide-mcu/src/firmware.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [firmware](/crates/oxide-mcu/src/firmware.md) |
