---
okf_version: "0.2"
type: Function
title: dispatch_edit_message
description: "Edit-command message handler (namespaced family, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/document.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/document/dispatch_edit_message_1
language: rust
---

# dispatch_edit_message

Edit-command message handler (namespaced family, ADR-0001 D3).

## Signature

```rust
pub(crate) fn dispatch_edit_message(&mut self, msg: EditMsg) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Edit-command message handler (namespaced family, ADR-0001 D3).
Delete / undo / redo / rotate / mirror / clipboard (copy, cut,
paste, smart-paste) / duplicate. Routed from `dispatch_update`
via `Message::Edit`.

## Source
Lines 10–78 in `crates/oxide-app/src/app/dispatch/document.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document](/crates/oxide-app/src/app/dispatch/document.md) |
