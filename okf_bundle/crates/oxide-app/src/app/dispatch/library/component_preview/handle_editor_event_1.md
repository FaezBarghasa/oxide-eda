---
okf_version: "0.2"
type: Function
title: handle_editor_event
description: Component Preview event handler.
resource: crates/oxide-app/src/app/dispatch/library/component_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/component_preview/handle_editor_event_1
language: rust
---

# handle_editor_event

Component Preview event handler.

## Signature

```rust
pub(super) fn handle_editor_event(
        &mut self,
        address: EditorAddress,
        msg: EditorMsg,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Component Preview event handler.

## Source
Lines 31–123 in `crates/oxide-app/src/app/dispatch/library/component_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component_preview](/crates/oxide-app/src/app/dispatch/library/component_preview.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
