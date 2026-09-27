---
okf_version: "0.2"
type: Function
title: rollup
description: "Walks every sheet in the ExportContext, aggregates components according"
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/rollup
language: rust
---

# rollup

Walks every sheet in the ExportContext, aggregates components according

## Signature

```rust
pub fn rollup(ctx: &ExportContext, opts: &BomOptions) -> BomTable
```

## Visibility

- `pub`

## Docstring

Walks every sheet in the ExportContext, aggregates components according
to the BomOptions, and returns a BomTable ready to emit.

## Source
Lines 155–160 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| calls | [build_bom_context](/crates/oxide-output/src/bom/mod/build_bom_context.md) |
| calls | [engine_options_from_opts](/crates/oxide-output/src/bom/mod/engine_options_from_opts.md) |
| calls | [build_table](/crates/oxide-bom/src/lib/build_table.md) |
| called_by | [rebuild_bom_table](/crates/oxide-app/src/app/handlers/menu/export/bom/rebuild_bom_table.md) |
| called_by | [qa_bom](/crates/oxide-output/examples/qa_harness/qa_bom.md) |
