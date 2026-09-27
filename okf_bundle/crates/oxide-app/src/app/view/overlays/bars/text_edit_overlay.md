---
okf_version: "0.2"
type: Function
title: text_edit_overlay
description: "In-canvas text-edit input — the floating `text_input` anchored on"
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/text_edit_overlay
language: rust
---

# text_edit_overlay

In-canvas text-edit input — the floating `text_input` anchored on

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn text_edit_overlay(&self) -> Option<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

In-canvas text-edit input — the floating `text_input` anchored on
top of the label being edited. Converts the object's world
position through the live camera into a window-absolute screen
position each frame.

## Source
Lines 568–639 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
