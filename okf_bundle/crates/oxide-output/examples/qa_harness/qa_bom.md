---
okf_version: "0.2"
type: Function
title: qa_bom
resource: crates/oxide-output/examples/qa_harness.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/examples/qa_harness/qa_bom
language: rust
---

# qa_bom

## Signature

```rust
fn qa_bom(ctx: &ExportContext, out_dir: &Path, grouping: BomGrouping, label: &str)
```

## Source
Lines 163–269 in `crates/oxide-output/examples/qa_harness.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qa_harness](/crates/oxide-output/examples/qa_harness.md) |
| calls | [rollup](/crates/oxide-output/src/bom/mod/rollup.md) |
| called_by | [main](/crates/oxide-output/examples/qa_harness/main.md) |
