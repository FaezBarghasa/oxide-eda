---
okf_version: "0.2"
type: Function
title: handle_update_drawing_field
resource: crates/oxide-app/src/app/handlers/editing_commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/editing_commands/handle_update_drawing_field
language: rust
---

# handle_update_drawing_field

## Signature

```rust
impl Oxide { pub(crate) fn handle_update_drawing_field(
        &mut self,
        target_uuid: uuid::Uuid,
        edit: crate::app::contracts::DrawingFieldEdit,
    ) -> iced::Task<crate::app::Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 152–188 in `crates/oxide-app/src/app/handlers/editing_commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editing_commands](/crates/oxide-app/src/app/handlers/editing_commands.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_drawing_edit](/crates/oxide-app/src/app/handlers/editing_commands/apply_drawing_edit.md) |
