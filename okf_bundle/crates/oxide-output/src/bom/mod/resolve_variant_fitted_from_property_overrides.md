---
okf_version: "0.2"
type: Function
title: resolve_variant_fitted_from_property_overrides
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/resolve_variant_fitted_from_property_overrides
language: rust
---

# resolve_variant_fitted_from_property_overrides

## Signature

```rust
fn resolve_variant_fitted_from_property_overrides(
    symbol: &oxide_types::schematic::Symbol,
    active_variant: &str,
) -> Option<bool>
```

## Source
Lines 363–384 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| calls | [property_variant_kind](/crates/oxide-output/src/bom/mod/property_variant_kind.md) |
| calls | [parse_bool_field](/crates/oxide-output/src/bom/mod/parse_bool_field.md) |
| called_by | [resolve_variant_fitted](/crates/oxide-output/src/bom/mod/resolve_variant_fitted.md) |
