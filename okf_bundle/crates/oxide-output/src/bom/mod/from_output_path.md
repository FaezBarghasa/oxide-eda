---
okf_version: "0.2"
type: Function
title: from_output_path
description: Resolve BOM format from output file extension.
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/from_output_path
language: rust
---

# from_output_path

Resolve BOM format from output file extension.

## Signature

```rust
impl BomFormat { pub fn from_output_path(path: &Path) -> Self }
```

## Visibility

- `pub`

## Docstring

Resolve BOM format from output file extension.
Falls back to CSV when extension is missing or unknown.

## Source
Lines 114–125 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
