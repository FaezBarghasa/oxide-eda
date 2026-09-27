---
okf_version: "0.2"
type: Function
title: view_resize_handle
resource: crates/oxide-app/src/app/view/chrome.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/chrome/view_resize_handle
language: rust
---

# view_resize_handle

## Signature

```rust
impl Oxide { pub(super) fn view_resize_handle(
        &self,
        target: DragTarget,
        visible: bool,
        horizontal: bool,
    ) -> Element<'_, Message> }
```

## Visibility

- `pub(super)`

## Source
Lines 574–605 in `crates/oxide-app/src/app/view/chrome.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chrome](/crates/oxide-app/src/app/view/chrome.md) |
| calls | [resize_handle](/crates/oxide-app/src/styles/resize_handle.md) |
