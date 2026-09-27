---
okf_version: "0.2"
type: Function
title: parse_variant_field_key
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/parse_variant_field_key
language: rust
---

# parse_variant_field_key

## Signature

```rust
fn parse_variant_field_key(key: &str) -> Option<(String, VariantFieldKind)>
```

## Source
Lines 260–313 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| called_by | [resolve_variant_fitted_from_fields](/crates/oxide-output/src/bom/mod/resolve_variant_fitted_from_fields.md) |
