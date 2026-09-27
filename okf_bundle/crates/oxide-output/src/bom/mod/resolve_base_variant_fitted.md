---
okf_version: "0.2"
type: Function
title: resolve_base_variant_fitted
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/resolve_base_variant_fitted
language: rust
---

# resolve_base_variant_fitted

## Signature

```rust
fn resolve_base_variant_fitted(symbol: &oxide_types::schematic::Symbol) -> Option<bool>
```

## Source
Lines 386–413 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| calls | [field_value_ci](/crates/oxide-output/src/bom/mod/field_value_ci.md) |
| calls | [parse_bool_field](/crates/oxide-output/src/bom/mod/parse_bool_field.md) |
| calls | [property_variant_kind](/crates/oxide-output/src/bom/mod/property_variant_kind.md) |
| calls | [rank_fitted_over_dnp](/crates/oxide-output/src/bom/mod/rank_fitted_over_dnp.md) |
| called_by | [resolve_variant_fitted](/crates/oxide-output/src/bom/mod/resolve_variant_fitted.md) |
