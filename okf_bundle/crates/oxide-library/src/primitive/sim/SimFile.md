---
okf_version: "0.2"
type: Class
title: SimFile
description: "`.snxsim` container — Altium parity for SimModel storage. The"
resource: crates/oxide-library/src/primitive/sim.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:11:54Z"
concept_id: crates/oxide-library/src/primitive/sim/SimFile
language: rust
---

# SimFile

`.snxsim` container — Altium parity for SimModel storage. The

## Signature

```rust
pub struct SimFile
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

`.snxsim` container — Altium parity for SimModel storage. The
envelope mirrors [`crate::primitive::SymbolFile`] /
[`crate::primitive::FootprintFile`] (file-level uuid + display
name + array-of-tables payload). Today the convention is one
SimModel per file; the Vec leaves room for multi-model SPICE
libraries without a wire-format break.

Wire format (v0.18.5): TOML manifest header + one `[[models]]`
entry per `SimModel`. Each entry's `body` field is emitted as a
`body = '''…'''` literal multi-line string so SPICE / Verilog-A
source is line-diffable in git output. Everything else (kind
enum, default_node_map, scalars) stays as inline TOML.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `format`
- `file_uuid`
- `display_name`
- `models`
- `created`
- `updated`

## Source
Lines 90–105 in `crates/oxide-library/src/primitive/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-library/src/primitive/sim.md) |
