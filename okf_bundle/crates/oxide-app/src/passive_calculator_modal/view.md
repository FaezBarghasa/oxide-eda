---
okf_version: "0.2"
type: Function
title: view
resource: crates/oxide-app/src/passive_calculator_modal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/passive_calculator_modal/view
language: rust
---

# view

## Signature

```rust
pub fn view(
    tokens: &'a ThemeTokens,
    theme_id: ThemeId,
    control: &'a CalculatorControl,
) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 28–74 in `crates/oxide-app/src/passive_calculator_modal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [passive_calculator_modal](/crates/oxide-app/src/passive_calculator_modal.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
