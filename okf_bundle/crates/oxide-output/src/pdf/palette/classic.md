---
okf_version: "0.2"
type: Function
title: classic
description: "Legacy default palette — cream paper, dark-blue wires, mustard"
resource: crates/oxide-output/src/pdf/palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/pdf/palette/classic
language: rust
---

# classic

Legacy default palette — cream paper, dark-blue wires, mustard

## Signature

```rust
impl SchematicPalette { pub fn classic() -> Self }
```

## Visibility

- `pub`

## Docstring

Legacy default palette — cream paper, dark-blue wires, mustard
symbol bodies. Preserved for tests and as the
default-for-tests `PdfOptions::default()` palette so the
existing /Page bytes don't shift under tests.

## Source
Lines 58–81 in `crates/oxide-output/src/pdf/palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [palette](/crates/oxide-output/src/pdf/palette.md) |
