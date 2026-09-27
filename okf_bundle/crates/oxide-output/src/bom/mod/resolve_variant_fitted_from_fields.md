---
okf_version: "0.2"
type: Function
title: resolve_variant_fitted_from_fields
description: "Fit state for `active_variant` taken from `symbol.fields`."
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/resolve_variant_fitted_from_fields
language: rust
---

# resolve_variant_fitted_from_fields

Fit state for `active_variant` taken from `symbol.fields`.

## Signature

```rust
fn resolve_variant_fitted_from_fields(
    symbol: &oxide_types::schematic::Symbol,
    active_variant: &str,
) -> Option<bool>
```

## Docstring

Fit state for `active_variant` taken from `symbol.fields`.

`fields` is a `HashMap`, so a symbol carrying contradictory entries
(`Fitted@LITE=yes` alongside `DNP@LITE=yes`) would resolve by hash order and
export a different fit state from one run to the next. Resolve with the same
precedence `resolve_base_variant_fitted` applies — Fitted ahead of DNP — and
break any remaining tie on the field key, so the answer is a pure function
of the file rather than of the allocator.

## Source
Lines 423–441 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| calls | [parse_variant_field_key](/crates/oxide-output/src/bom/mod/parse_variant_field_key.md) |
| calls | [parse_bool_field](/crates/oxide-output/src/bom/mod/parse_bool_field.md) |
| calls | [rank_fitted_over_dnp](/crates/oxide-output/src/bom/mod/rank_fitted_over_dnp.md) |
| called_by | [resolve_variant_fitted](/crates/oxide-output/src/bom/mod/resolve_variant_fitted.md) |
