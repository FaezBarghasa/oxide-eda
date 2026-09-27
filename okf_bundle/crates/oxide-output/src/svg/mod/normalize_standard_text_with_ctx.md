---
okf_version: "0.2"
type: Function
title: normalize_standard_text_with_ctx
resource: crates/oxide-output/src/svg/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/mod/normalize_standard_text_with_ctx
language: rust
---

# normalize_standard_text_with_ctx

## Signature

```rust
fn normalize_standard_text_with_ctx(input: &str, ctx: &ExpressionEvalContext<'_>) -> String
```

## Source
Lines 109–114 in `crates/oxide-output/src/svg/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [svg](/crates/oxide-output/src/svg/mod.md) |
| calls | [evaluate_expressions](/crates/oxide-types/src/markup/evaluate_expressions.md) |
| called_by | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
| called_by | [normalize_standard_text](/crates/oxide-output/src/svg/mod/normalize_standard_text.md) |
| called_by | [push_symbol_pins](/crates/oxide-output/src/svg/symbols/push_symbol_pins.md) |
