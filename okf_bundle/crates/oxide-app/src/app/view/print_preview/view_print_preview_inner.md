---
okf_version: "0.2"
type: Function
title: view_print_preview_inner
resource: crates/oxide-app/src/app/view/print_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/print_preview/view_print_preview_inner
language: rust
---

# view_print_preview_inner

## Signature

```rust
impl Oxide { fn view_print_preview_inner(&self, draggable: bool) -> Element<'_, Message> }
```

## Source
Lines 229–325 in `crates/oxide-app/src/app/view/print_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [print_preview](/crates/oxide-app/src/app/view/print_preview.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| calls | [draggable_header](/crates/oxide-app/src/app/view/dialogs/widgets/draggable_header.md) |
| calls | [detached_header](/crates/oxide-app/src/app/view/dialogs/widgets/detached_header.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
