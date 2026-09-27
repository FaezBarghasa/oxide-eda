---
okf_version: "0.2"
type: Function
title: build_bom_context
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/build_bom_context
language: rust
---

# build_bom_context

## Signature

```rust
fn build_bom_context(ctx: &ExportContext, opts: &BomOptions) -> BomContext
```

## Source
Lines 173–234 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| calls | [resolve_variant_fitted](/crates/oxide-output/src/bom/mod/resolve_variant_fitted.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [export](/crates/oxide-output/src/bom/mod/export.md) |
| called_by | [rollup](/crates/oxide-output/src/bom/mod/rollup.md) |
