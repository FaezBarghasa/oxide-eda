---
okf_version: "0.2"
type: Function
title: resolve_dim
description: "Resolve a [`DimTarget`] against the parameter table."
resource: crates/oxide-sketch/src/solver/residual.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residual/resolve_dim
language: rust
---

# resolve_dim

Resolve a [`DimTarget`] against the parameter table.

## Signature

```rust
pub fn resolve_dim(target: &DimTarget, params: &ResolvedParams) -> Result<f64, SketchError>
```

## Visibility

- `pub`

## Docstring

Resolve a [`DimTarget`] against the parameter table.

`Literal` values pass through unchanged. `Expr` strings are
parsed and evaluated through the [`crate::expr`] machinery —
supporting arithmetic, comparisons, ternaries, and `lookup(...)`
calls. The optional Altium-style `=` prefix is stripped before
parsing.

`ResolvedParams` values are injected into the eval context as
`Literal(Quantity::length(v))`. This works cleanly for length-
family expressions (Distance constraints) where each parameter
is a length in mm. Angle-family parameters work the same way at
the bare-name lookup level (`= apex_angle`); for parameter-driven
arithmetic on angles, write the literal unit explicitly
(e.g. `= apex_angle * 1rad / 1rad` reduces back to the raw
canonical value, but the more idiomatic form is to keep the
expression inline rather than chained through a Length-typed
parameter table).

## Source
Lines 56–95 in `crates/oxide-sketch/src/solver/residual.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [residual](/crates/oxide-sketch/src/solver/residual.md) |
| calls | [eval_expr](/crates/oxide-erc-dsl/src/compiler/eval_expr.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
