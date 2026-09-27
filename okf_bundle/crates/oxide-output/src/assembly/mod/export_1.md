---
okf_version: "0.2"
type: Function
title: export
description: Export Pick-and-Place CPL / Centroid data from a PCB board.
resource: crates/oxide-output/src/assembly/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:56:29Z"
concept_id: crates/oxide-output/src/assembly/mod/export_1
language: rust
---

# export

Export Pick-and-Place CPL / Centroid data from a PCB board.

## Signature

```rust
pub fn export(&self, board: &PcbBoard) -> Result<String, AssemblyError>
```

## Visibility

- `pub`

## Docstring

Export Pick-and-Place CPL / Centroid data from a PCB board.

## Source
Lines 55–113 in `crates/oxide-output/src/assembly/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [assembly](/crates/oxide-output/src/assembly/mod.md) |
