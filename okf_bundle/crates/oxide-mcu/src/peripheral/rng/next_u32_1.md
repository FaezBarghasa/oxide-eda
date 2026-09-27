---
okf_version: "0.2"
type: Function
title: next_u32
description: "Generates next 32-bit random integer using xorshift64* pseudo-entropy."
resource: crates/oxide-mcu/src/peripheral/rng.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:38Z"
concept_id: crates/oxide-mcu/src/peripheral/rng/next_u32_1
language: rust
---

# next_u32

Generates next 32-bit random integer using xorshift64* pseudo-entropy.

## Signature

```rust
pub fn next_u32(&mut self) -> Option<u32>
```

## Visibility

- `pub`

## Docstring

Generates next 32-bit random integer using xorshift64* pseudo-entropy.

## Source
Lines 31–43 in `crates/oxide-mcu/src/peripheral/rng.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rng](/crates/oxide-mcu/src/peripheral/rng.md) |
