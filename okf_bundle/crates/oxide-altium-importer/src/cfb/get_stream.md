---
okf_version: "0.2"
type: Function
title: get_stream
description: Read and optionally decompress a named stream.
resource: crates/oxide-altium-importer/src/cfb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:45:25Z"
concept_id: crates/oxide-altium-importer/src/cfb/get_stream
language: rust
---

# get_stream

Read and optionally decompress a named stream.

## Signature

```rust
impl CfbContainer { pub fn get_stream(&self, name: &str) -> Result<Vec<u8>, AltiumImportError> }
```

## Visibility

- `pub`

## Docstring

Read and optionally decompress a named stream.

## Source
Lines 177–193 in `crates/oxide-altium-importer/src/cfb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cfb](/crates/oxide-altium-importer/src/cfb.md) |
