---
okf_version: "0.2"
type: Function
title: primary_button
resource: crates/chrome-catalog/src/bom_modal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:chrome-catalog"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/chrome-catalog/src/bom_modal/primary_button
language: rust
---

# primary_button

## Signature

```rust
fn primary_button(label: &str, accent: Color, text_color: Color) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Source
Lines 210–223 in `crates/chrome-catalog/src/bom_modal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom_modal](/crates/chrome-catalog/src/bom_modal.md) |
| calls | [color](/crates/chrome-catalog/src/theme/color.md) |
