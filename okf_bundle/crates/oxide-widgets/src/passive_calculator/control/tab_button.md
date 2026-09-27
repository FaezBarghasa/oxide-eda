---
okf_version: "0.2"
type: Function
title: tab_button
resource: crates/oxide-widgets/src/passive_calculator/control.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/passive_calculator/control/tab_button
language: rust
---

# tab_button

## Signature

```rust
fn tab_button(
    label: &'a str,
    active: bool,
    message: CalculatorMessage,
    tokens: &'a ThemeTokens,
) -> Element<'a, CalculatorMessage>
```

## Type Parameters

- `'a`

## Source
Lines 398–431 in `crates/oxide-widgets/src/passive_calculator/control.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [control](/crates/oxide-widgets/src/passive_calculator/control.md) |
| calls | [token_color](/crates/oxide-widgets/src/passive_calculator/control/token_color.md) |
| called_by | [view](/crates/oxide-widgets/src/passive_calculator/control/view.md) |
