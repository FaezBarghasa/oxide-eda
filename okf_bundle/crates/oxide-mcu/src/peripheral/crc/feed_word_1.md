---
okf_version: "0.2"
type: Function
title: feed_word
description: Feeds 32-bit word into CRC calculation unit.
resource: crates/oxide-mcu/src/peripheral/crc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:33Z"
concept_id: crates/oxide-mcu/src/peripheral/crc/feed_word_1
language: rust
---

# feed_word

Feeds 32-bit word into CRC calculation unit.

## Signature

```rust
pub fn feed_word(&mut self, mut data: u32) -> u32
```

## Visibility

- `pub`

## Docstring

Feeds 32-bit word into CRC calculation unit.

## Source
Lines 38–55 in `crates/oxide-mcu/src/peripheral/crc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crc](/crates/oxide-mcu/src/peripheral/crc.md) |
