---
okf_version: "0.2"
type: Function
title: result_view
resource: crates/oxide-widgets/src/passive_calculator/control.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/passive_calculator/control/result_view
language: rust
---

# result_view

## Signature

```rust
impl CalculatorControl { fn result_view(
        &'a self,
        result: &'a Network,
        tokens: &'a ThemeTokens,
    ) -> Element<'a, CalculatorMessage> }
```

## Type Parameters

- `'a`

## Source
Lines 318–385 in `crates/oxide-widgets/src/passive_calculator/control.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [control](/crates/oxide-widgets/src/passive_calculator/control.md) |
| calls | [token_color](/crates/oxide-widgets/src/passive_calculator/control/token_color.md) |
| calls | [panel_style](/crates/oxide-widgets/src/passive_calculator/control/panel_style.md) |
