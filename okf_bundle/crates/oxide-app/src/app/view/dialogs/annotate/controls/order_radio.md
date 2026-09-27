---
okf_version: "0.2"
type: Function
title: order_radio
resource: crates/oxide-app/src/app/view/dialogs/annotate/controls.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate/controls/order_radio
language: rust
---

# order_radio

## Signature

```rust
pub(super) fn order_radio(
    label: &str,
    value: AnnotateOrder,
    current: AnnotateOrder,
    text_c: Color,
    border: Color,
) -> Element<'_, Message>
```

## Visibility

- `pub(super)`

## Source
Lines 132–159 in `crates/oxide-app/src/app/view/dialogs/annotate/controls.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [controls](/crates/oxide-app/src/app/view/dialogs/annotate/controls.md) |
