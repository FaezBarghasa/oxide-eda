---
okf_version: "0.2"
type: Function
title: parse_bool_field
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/parse_bool_field
language: rust
---

# parse_bool_field

## Signature

```rust
fn parse_bool_field(raw: &str) -> Option<bool>
```

## Source
Lines 252–258 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| called_by | [resolve_base_variant_fitted](/crates/oxide-output/src/bom/mod/resolve_base_variant_fitted.md) |
| called_by | [resolve_variant_fitted_from_fields](/crates/oxide-output/src/bom/mod/resolve_variant_fitted_from_fields.md) |
| called_by | [resolve_variant_fitted_from_property_overrides](/crates/oxide-output/src/bom/mod/resolve_variant_fitted_from_property_overrides.md) |
