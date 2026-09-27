---
okf_version: "0.2"
type: Function
title: parse
description: Parse a Compound File Binary container from a raw byte slice.
resource: crates/oxide-altium-importer/src/cfb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:45:25Z"
concept_id: crates/oxide-altium-importer/src/cfb/parse_1
language: rust
---

# parse

Parse a Compound File Binary container from a raw byte slice.

## Signature

```rust
pub fn parse(data: &[u8]) -> Result<Self, AltiumImportError>
```

## Visibility

- `pub`

## Docstring

Parse a Compound File Binary container from a raw byte slice.

## Source
Lines 23–174 in `crates/oxide-altium-importer/src/cfb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cfb](/crates/oxide-altium-importer/src/cfb.md) |
| calls | [truncate](/crates/oxide-output/examples/qa_harness/truncate.md) |
