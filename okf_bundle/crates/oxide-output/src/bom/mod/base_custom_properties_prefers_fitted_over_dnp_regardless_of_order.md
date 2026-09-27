---
okf_version: "0.2"
type: Function
title: base_custom_properties_prefers_fitted_over_dnp_regardless_of_order
description: "`resolve_base_variant_fitted`'s `custom_properties` loop used to return"
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/base_custom_properties_prefers_fitted_over_dnp_regardless_of_order
language: rust
---

# base_custom_properties_prefers_fitted_over_dnp_regardless_of_order

`resolve_base_variant_fitted`'s `custom_properties` loop used to return

## Signature

```rust
fn base_custom_properties_prefers_fitted_over_dnp_regardless_of_order()
```

## Decorators

- `test`

## Docstring

`resolve_base_variant_fitted`'s `custom_properties` loop used to return
on the first parseable match with no ranking, so a symbol listing DNP
before Fitted resolved as DNP — disagreeing with the variant path,
which always ranks Fitted ahead of DNP. Both orderings must now agree.
[test]

## Source
Lines 634–660 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| calls | [test_symbol](/crates/oxide-output/src/bom/mod/test_symbol.md) |
