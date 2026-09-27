---
okf_version: "0.2"
type: Class
title: FirmwareImage
description: Parsed Firmware Metadata and verification summary.
resource: crates/oxide-mcu/src/firmware.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:44:25Z"
concept_id: crates/oxide-mcu/src/firmware/FirmwareImage
language: rust
---

# FirmwareImage

Parsed Firmware Metadata and verification summary.

## Signature

```rust
pub struct FirmwareImage
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Parsed Firmware Metadata and verification summary.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `path`
- `target`
- `entry_point`
- `segments`
- `total_size_bytes`

## Source
Lines 547–553 in `crates/oxide-mcu/src/firmware.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [firmware](/crates/oxide-mcu/src/firmware.md) |
