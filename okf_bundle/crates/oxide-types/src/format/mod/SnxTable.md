---
okf_version: "0.2"
type: Class
title: SnxTable
description: A row schema for TSV bulk blocks.
resource: crates/oxide-types/src/format/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/format/mod/SnxTable
language: rust
---

# SnxTable

A row schema for TSV bulk blocks.

## Signature

```rust
pub trait SnxTable
```

## Visibility

- `pub`

## Docstring

A row schema for TSV bulk blocks.

Implementors describe the TSV column order, how to render an
in-memory row to its column-cell strings, and how to parse a vector
of cell `&str` slices back into a row.

## Source
Lines 119–131 in `crates/oxide-types/src/format/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [format](/crates/oxide-types/src/format/mod.md) |
