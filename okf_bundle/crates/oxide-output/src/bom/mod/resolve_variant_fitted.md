---
okf_version: "0.2"
type: Function
title: resolve_variant_fitted
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/resolve_variant_fitted
language: rust
---

# resolve_variant_fitted

## Signature

```rust
fn resolve_variant_fitted(
    symbol: &oxide_types::schematic::Symbol,
    active_variant: Option<&str>,
) -> Option<bool>
```

## Source
Lines 443–462 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| calls | [resolve_variant_fitted_from_property_overrides](/crates/oxide-output/src/bom/mod/resolve_variant_fitted_from_property_overrides.md) |
| calls | [resolve_variant_fitted_from_fields](/crates/oxide-output/src/bom/mod/resolve_variant_fitted_from_fields.md) |
| calls | [resolve_base_variant_fitted](/crates/oxide-output/src/bom/mod/resolve_base_variant_fitted.md) |
| called_by | [build_bom_context](/crates/oxide-output/src/bom/mod/build_bom_context.md) |
