---
okf_version: "0.2"
type: Function
title: handle_print_preview_close
resource: crates/oxide-app/src/app/handlers/menu/export/print_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_close_1
language: rust
---

# handle_print_preview_close

## Signature

```rust
pub(crate) fn handle_print_preview_close(&mut self) -> iced::Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 269–279 in `crates/oxide-app/src/app/handlers/menu/export/print_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [print_preview](/crates/oxide-app/src/app/handlers/menu/export/print_preview.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
