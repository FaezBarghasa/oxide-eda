---
okf_version: "0.2"
type: Function
title: view_print_preview_body
description: "Detached-window flavour — bare body, no backdrop, no in-window"
resource: crates/oxide-app/src/app/view/print_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/print_preview/view_print_preview_body_1
language: rust
---

# view_print_preview_body

Detached-window flavour — bare body, no backdrop, no in-window

## Signature

```rust
pub(super) fn view_print_preview_body(&self) -> Element<'_, Message>
```

## Visibility

- `pub(super)`

## Docstring

Detached-window flavour — bare body, no backdrop, no in-window
drag handler (the OS window-drag covers the header).

## Source
Lines 225–227 in `crates/oxide-app/src/app/view/print_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [print_preview](/crates/oxide-app/src/app/view/print_preview.md) |
