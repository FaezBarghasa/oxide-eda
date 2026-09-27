---
okf_version: "0.2"
type: Function
title: order_preview
description: "Compact visual *legend* of the annotate order — this is intentionally a"
resource: crates/oxide-app/src/app/view/dialogs/annotate/controls.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate/controls/order_preview
language: rust
---

# order_preview

Compact visual *legend* of the annotate order — this is intentionally a

## Signature

```rust
pub(super) fn order_preview(
    order: AnnotateOrder,
    text_c: Color,
    text_muted: Color,
    border: Color,
) -> Element<'static, Message>
```

## Visibility

- `pub(super)`

## Docstring

Compact visual *legend* of the annotate order — this is intentionally a
static R1..R4 diagram that illustrates how four parts arranged in a 2×2
grid would be numbered under the selected traversal. It does NOT reflect
the user's actual components; it's the same convention Altium uses.

## Source
Lines 50–118 in `crates/oxide-app/src/app/view/dialogs/annotate/controls.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [controls](/crates/oxide-app/src/app/view/dialogs/annotate/controls.md) |
| calls | [bordered_style](/crates/oxide-app/src/app/view/dialogs/annotate/controls/bordered_style.md) |
