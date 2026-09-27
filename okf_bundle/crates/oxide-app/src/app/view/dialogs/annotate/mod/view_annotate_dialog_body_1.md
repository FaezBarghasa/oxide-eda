---
okf_version: "0.2"
type: Function
title: view_annotate_dialog_body
description: "Detached-window flavour — just the body, no backdrop, no drag"
resource: crates/oxide-app/src/app/view/dialogs/annotate/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate/mod/view_annotate_dialog_body_1
language: rust
---

# view_annotate_dialog_body

Detached-window flavour — just the body, no backdrop, no drag

## Signature

```rust
pub(in crate::app::view) fn view_annotate_dialog_body(&self) -> Element<'_, Message>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Detached-window flavour — just the body, no backdrop, no drag
handler on the header (the OS window chrome owns that).

## Source
Lines 44–46 in `crates/oxide-app/src/app/view/dialogs/annotate/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate](/crates/oxide-app/src/app/view/dialogs/annotate/mod.md) |
