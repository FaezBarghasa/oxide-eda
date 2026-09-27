---
okf_version: "0.2"
type: Function
title: export
resource: crates/oxide-output/src/netlist/pspice.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:17:11Z"
concept_id: crates/oxide-output/src/netlist/pspice/export
language: rust
---

# export

## Signature

```rust
impl PSpiceNetlistExporter { fn export(
        &self,
        ctx: &ExportContext,
        opts: &Self::Options,
    ) -> Result<Self::Output, Self::Error> }
```

## Source
Lines 34–56 in `crates/oxide-output/src/netlist/pspice.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pspice](/crates/oxide-output/src/netlist/pspice.md) |
