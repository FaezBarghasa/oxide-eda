---
okf_version: "0.2"
type: Function
title: resize_handle
description: Resize handle between panels (thin draggable border)
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/resize_handle
language: rust
---

# resize_handle

Resize handle between panels (thin draggable border)

## Signature

```rust
pub fn resize_handle(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Resize handle between panels (thin draggable border)

## Source
Lines 173–179 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_resize_handle](/crates/oxide-app/src/app/view/chrome/view_resize_handle.md) |
