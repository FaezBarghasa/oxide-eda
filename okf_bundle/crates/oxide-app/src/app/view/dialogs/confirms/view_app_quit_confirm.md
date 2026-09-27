---
okf_version: "0.2"
type: Function
title: view_app_quit_confirm
resource: crates/oxide-app/src/app/view/dialogs/confirms.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/confirms/view_app_quit_confirm
language: rust
---

# view_app_quit_confirm

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn view_app_quit_confirm(&self) -> Element<'_, Message> }
```

## Visibility

- `pub(in crate::app::view)`

## Source
Lines 269–275 in `crates/oxide-app/src/app/view/dialogs/confirms.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [confirms](/crates/oxide-app/src/app/view/dialogs/confirms.md) |
| calls | [wrap_modal](/crates/oxide-app/src/app/view/dialogs/widgets/wrap_modal.md) |
