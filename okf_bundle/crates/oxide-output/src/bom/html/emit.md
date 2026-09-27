---
okf_version: "0.2"
type: Function
title: emit
description: Emit a BOM table as self-contained HTML with inline CSS.
resource: crates/oxide-output/src/bom/html.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/bom/html/emit
language: rust
---

# emit

Emit a BOM table as self-contained HTML with inline CSS.

## Signature

```rust
pub fn emit(table: &BomTable, columns: &[BomColumn]) -> Result<Vec<u8>, BomError>
```

## Visibility

- `pub`

## Docstring

Emit a BOM table as self-contained HTML with inline CSS.

## Source
Lines 8–134 in `crates/oxide-output/src/bom/html.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [html](/crates/oxide-output/src/bom/html.md) |
