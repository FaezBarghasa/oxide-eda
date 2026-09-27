---
okf_version: "0.2"
type: Function
title: parse_pspice_library
description: "Parse a PSpice library text payload containing one or more `.SUBCKT` blocks or `.MODEL` statements."
resource: crates/oxide-library/src/primitive/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:11:54Z"
concept_id: crates/oxide-library/src/primitive/sim/parse_pspice_library
language: rust
---

# parse_pspice_library

Parse a PSpice library text payload containing one or more `.SUBCKT` blocks or `.MODEL` statements.

## Signature

```rust
pub fn parse_pspice_library(source: &str) -> Vec<SimModel>
```

## Visibility

- `pub`

## Docstring

Parse a PSpice library text payload containing one or more `.SUBCKT` blocks or `.MODEL` statements.
Returns a list of extracted [`SimModel`] instances with kind set to [`SimKind::PSpice`].

## Source
Lines 259–328 in `crates/oxide-library/src/primitive/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-library/src/primitive/sim.md) |
| called_by | [parse_pspice_library_subckt_and_model](/crates/oxide-library/src/primitive/sim/parse_pspice_library_subckt_and_model.md) |
