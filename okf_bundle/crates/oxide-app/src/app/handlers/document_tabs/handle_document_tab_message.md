---
okf_version: "0.2"
type: Function
title: handle_document_tab_message
resource: crates/oxide-app/src/app/handlers/document_tabs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_tabs/handle_document_tab_message
language: rust
---

# handle_document_tab_message

## Signature

```rust
impl Oxide { pub(crate) fn handle_document_tab_message(
        &mut self,
        window_id: iced::window::Id,
        msg: TabMessage,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 6–95 in `crates/oxide-app/src/app/handlers/document_tabs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document_tabs](/crates/oxide-app/src/app/handlers/document_tabs.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
