---
okf_version: "0.2"
type: Function
title: smd_pad
resource: crates/oxide-app/examples/build_stock_library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/examples/build_stock_library/smd_pad
language: rust
---

# smd_pad

## Signature

```rust
fn smd_pad(
    plane: PlaneId,
    anchor_x: f64,
    anchor_y: f64,
    number: &str,
    size_x_expr: &str,
    size_y_expr: &str,
    offset_x_expr: Option<&str>,
    offset_y_expr: Option<&str>,
    rotation_expr: Option<&str>,
    side: PadSide,
    shape: PadShape,
    mask_margin_expr: Option<&str>,
    paste_margin_expr: Option<&str>,
) -> Entity
```

## Decorators

- `expect(
    clippy::too_many_arguments,
    reason = "13 arguments: an example builder that mirrors the stock-library row schema one field per parameter"
)`

## Source
Lines 98–138 in `crates/oxide-app/examples/build_stock_library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [build_stock_library](/crates/oxide-app/examples/build_stock_library.md) |
| called_by | [build_qfn16](/crates/oxide-app/examples/build_stock_library/build_qfn16.md) |
| called_by | [build_r0805](/crates/oxide-app/examples/build_stock_library/build_r0805.md) |
| called_by | [build_soic8](/crates/oxide-app/examples/build_stock_library/build_soic8.md) |
