---
okf_version: "0.2"
type: Function
title: build_validation_context
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/build_validation_context
language: rust
---

# build_validation_context

## Signature

```rust
fn build_validation_context(ctx: &BomContext, opts: &BomOptions) -> BomContext
```

## Source
Lines 240–250 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| calls | [component_is_exported](/crates/oxide-output/src/bom/mod/component_is_exported.md) |
| called_by | [export](/crates/oxide-output/src/bom/mod/export.md) |
