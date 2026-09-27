---
okf_version: "0.2"
type: Function
title: push_symbol_pins
resource: crates/oxide-output/src/svg/symbols.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/symbols/push_symbol_pins
language: rust
---

# push_symbol_pins

## Signature

```rust
pub(super) fn push_symbol_pins(
    out: &mut Vec<SvgElement>,
    sym: &Symbol,
    lib: &LibSymbol,
    xform: &PageTransform,
    eval_ctx: &ExpressionEvalContext<'_>,
    pin_net_names: Option<&HashMap<String, String>>,
    palette: &SchematicPalette,
)
```

## Visibility

- `pub(super)`

## Source
Lines 241–401 in `crates/oxide-output/src/svg/symbols.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbols](/crates/oxide-output/src/svg/symbols.md) |
| calls | [pin_direction](/crates/oxide-output/src/svg/symbols/pin_direction.md) |
| calls | [symbol_world_point](/crates/oxide-output/src/svg/symbols/symbol_world_point.md) |
| calls | [normalize_standard_text_with_ctx](/crates/oxide-output/src/svg/mod/normalize_standard_text_with_ctx.md) |
| called_by | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
