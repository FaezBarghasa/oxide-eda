---
okf_version: "0.2"
type: Function
title: bordered_style
resource: crates/oxide-app/src/app/view/dialogs/annotate/controls.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate/controls/bordered_style
language: rust
---

# bordered_style

## Signature

```rust
pub(super) fn bordered_style(border: Color) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub(super)`

## Source
Lines 120–130 in `crates/oxide-app/src/app/view/dialogs/annotate/controls.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [controls](/crates/oxide-app/src/app/view/dialogs/annotate/controls.md) |
| called_by | [order_preview](/crates/oxide-app/src/app/view/dialogs/annotate/controls/order_preview.md) |
| called_by | [view_annotate_dialog_body_inner](/crates/oxide-app/src/app/view/dialogs/annotate/mod/view_annotate_dialog_body_inner.md) |
