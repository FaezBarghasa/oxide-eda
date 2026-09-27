---
okf_version: "0.2"
type: Function
title: export
resource: crates/oxide-output/src/netlist/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T19:55:41Z"
concept_id: crates/oxide-output/src/netlist/mod/export_1
language: rust
---

# export

## Signature

```rust
fn export(
        &self,
        ctx: &ExportContext,
        opts: &Self::Options,
    ) -> Result<Self::Output, Self::Error>
```

## Source
Lines 57–66 in `crates/oxide-output/src/netlist/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [netlist](/crates/oxide-output/src/netlist/mod.md) |
| calls | [render_listing](/crates/oxide-output/src/netlist/mod/render_listing.md) |
