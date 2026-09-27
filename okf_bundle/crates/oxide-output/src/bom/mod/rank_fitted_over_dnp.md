---
okf_version: "0.2"
type: Function
title: rank_fitted_over_dnp
description: Rank a parsed fit-state field so Fitted always outranks DNP on a tie.
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/rank_fitted_over_dnp
language: rust
---

# rank_fitted_over_dnp

Rank a parsed fit-state field so Fitted always outranks DNP on a tie.

## Signature

```rust
fn rank_fitted_over_dnp(kind: VariantFieldKind, parsed: bool) -> (u8, bool)
```

## Docstring

Rank a parsed fit-state field so Fitted always outranks DNP on a tie.

Shared by every path that must arbitrate more than one candidate field for
the same symbol, so a Fitted-vs-DNP conflict resolves the same way
regardless of which path resolved it — encoding the precedence twice is
exactly how the base `custom_properties` path and the variant `fields`
path drifted apart.

## Source
Lines 332–337 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| called_by | [resolve_base_variant_fitted](/crates/oxide-output/src/bom/mod/resolve_base_variant_fitted.md) |
| called_by | [resolve_variant_fitted_from_fields](/crates/oxide-output/src/bom/mod/resolve_variant_fitted_from_fields.md) |
