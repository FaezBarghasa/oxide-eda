---
okf_version: "0.2"
type: Function
title: export
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/export_1
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
Lines 473–493 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| calls | [build_bom_context](/crates/oxide-output/src/bom/mod/build_bom_context.md) |
| calls | [build_table](/crates/oxide-bom/src/lib/build_table.md) |
| calls | [engine_options_from_opts](/crates/oxide-output/src/bom/mod/engine_options_from_opts.md) |
| calls | [build_validation_context](/crates/oxide-output/src/bom/mod/build_validation_context.md) |
| calls | [validate_table](/crates/oxide-bom/src/lib/validate_table.md) |
