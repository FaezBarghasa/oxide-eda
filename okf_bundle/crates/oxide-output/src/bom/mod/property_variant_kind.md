---
okf_version: "0.2"
type: Function
title: property_variant_kind
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/property_variant_kind
language: rust
---

# property_variant_kind

## Signature

```rust
fn property_variant_kind(key: &str) -> Option<VariantFieldKind>
```

## Source
Lines 315–323 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| called_by | [resolve_base_variant_fitted](/crates/oxide-output/src/bom/mod/resolve_base_variant_fitted.md) |
| called_by | [resolve_variant_fitted_from_property_overrides](/crates/oxide-output/src/bom/mod/resolve_variant_fitted_from_property_overrides.md) |
