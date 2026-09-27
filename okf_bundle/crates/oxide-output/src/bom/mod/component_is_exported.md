---
okf_version: "0.2"
type: Function
title: component_is_exported
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/component_is_exported
language: rust
---

# component_is_exported

## Signature

```rust
fn component_is_exported(component: &BomComponent, opts: &BomOptions) -> bool
```

## Source
Lines 236–238 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| called_by | [build_validation_context](/crates/oxide-output/src/bom/mod/build_validation_context.md) |
