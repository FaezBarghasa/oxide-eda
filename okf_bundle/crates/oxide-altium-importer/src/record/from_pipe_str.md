---
okf_version: "0.2"
type: Function
title: from_pipe_str
description: "Parse a single pipe-delimited string (e.g. `|RECORD=1|LOCATION.X=100|...`) into an [`AltiumRecord`]."
resource: crates/oxide-altium-importer/src/record.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:45:59Z"
concept_id: crates/oxide-altium-importer/src/record/from_pipe_str
language: rust
---

# from_pipe_str

Parse a single pipe-delimited string (e.g. `|RECORD=1|LOCATION.X=100|...`) into an [`AltiumRecord`].

## Signature

```rust
impl AltiumRecord { pub fn from_pipe_str(s: &str) -> Self }
```

## Visibility

- `pub`

## Docstring

Parse a single pipe-delimited string (e.g. `|RECORD=1|LOCATION.X=100|...`) into an [`AltiumRecord`].

## Source
Lines 49–57 in `crates/oxide-altium-importer/src/record.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [record](/crates/oxide-altium-importer/src/record.md) |
