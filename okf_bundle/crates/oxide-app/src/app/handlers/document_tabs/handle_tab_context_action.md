---
okf_version: "0.2"
type: Function
title: handle_tab_context_action
resource: crates/oxide-app/src/app/handlers/document_tabs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_tabs/handle_tab_context_action
language: rust
---

# handle_tab_context_action

## Signature

```rust
impl Oxide { pub(crate) fn handle_tab_context_action(
        &mut self,
        action: crate::app::TabContextAction,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 192–239 in `crates/oxide-app/src/app/handlers/document_tabs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document_tabs](/crates/oxide-app/src/app/handlers/document_tabs.md) |
