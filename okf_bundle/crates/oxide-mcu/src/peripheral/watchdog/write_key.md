---
okf_version: "0.2"
type: Function
title: write_key
description: "Key write emulation: `0xCCCC` starts watchdog, `0xAAAA` reloads counter."
resource: crates/oxide-mcu/src/peripheral/watchdog.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:20Z"
concept_id: crates/oxide-mcu/src/peripheral/watchdog/write_key
language: rust
---

# write_key

Key write emulation: `0xCCCC` starts watchdog, `0xAAAA` reloads counter.

## Signature

```rust
impl Iwdg { pub fn write_key(&mut self, key: u16) }
```

## Visibility

- `pub`

## Docstring

Key write emulation: `0xCCCC` starts watchdog, `0xAAAA` reloads counter.

## Source
Lines 33–44 in `crates/oxide-mcu/src/peripheral/watchdog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [watchdog](/crates/oxide-mcu/src/peripheral/watchdog.md) |
