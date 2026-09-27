---
okf_version: "0.2"
type: Function
title: bake_linear
resource: crates/oxide-bake/src/array/linear.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/array/linear/bake_linear
language: rust
---

# bake_linear

## Signature

```rust
pub(super) fn bake_linear(
    source: SketchEntityId,
    count_expr: &str,
    dx_expr: &str,
    dy_expr: &str,
    numbering: &NumberingScheme,
    params_ast: &BTreeMap<String, ExprNode>,
    sketch: &SketchData,
    solve: &FullSolveOutput,
    out: &mut Vec<LibPad>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Decorators

- `expect(
    clippy::too_many_arguments,
    reason = "linear-array placement takes the whole geometry + numbering spec at once"
)`

## Visibility

- `pub(super)`

## Source
Lines 28–125 in `crates/oxide-bake/src/array/linear.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linear](/crates/oxide-bake/src/array/linear.md) |
| calls | [derive_pad_number](/crates/oxide-bake/src/array/numbering/derive_pad_number.md) |
| calls | [bake_one_pad](/crates/oxide-bake/src/pad/bake_one_pad.md) |
| called_by | [bake_arrays](/crates/oxide-bake/src/array/mod/bake_arrays.md) |
