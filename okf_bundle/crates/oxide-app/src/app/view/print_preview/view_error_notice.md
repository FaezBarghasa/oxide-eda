---
okf_version: "0.2"
type: Function
title: view_error_notice
description: "Export-error modal — plain \"something went wrong, here's the"
resource: crates/oxide-app/src/app/view/print_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/print_preview/view_error_notice
language: rust
---

# view_error_notice

Export-error modal — plain "something went wrong, here's the

## Signature

```rust
impl Oxide { pub(super) fn view_error_notice(&self) -> Element<'_, Message> }
```

## Visibility

- `pub(super)`

## Docstring

Export-error modal — plain "something went wrong, here's the
message" dialog with an OK button. Sits on top of the print-preview
overlay when both would otherwise render; dismiss_layer handles
click-outside-to-close.

## Source
Lines 14–78 in `crates/oxide-app/src/app/view/print_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [print_preview](/crates/oxide-app/src/app/view/print_preview.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
