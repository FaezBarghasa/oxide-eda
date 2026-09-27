---
okf_version: "0.2"
type: Function
title: export_single_layer
description: Export a specific single layer.
resource: crates/oxide-output/src/gerber/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:02:33Z"
concept_id: crates/oxide-output/src/gerber/mod/export_single_layer
language: rust
---

# export_single_layer

Export a specific single layer.

## Signature

```rust
impl GerberExporter { pub fn export_single_layer(
        &self,
        board: &PcbBoard,
        layer: GerberLayer,
    ) -> Result<String, GerberError> }
```

## Visibility

- `pub`

## Docstring

Export a specific single layer.

## Source
Lines 155–430 in `crates/oxide-output/src/gerber/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gerber](/crates/oxide-output/src/gerber/mod.md) |
