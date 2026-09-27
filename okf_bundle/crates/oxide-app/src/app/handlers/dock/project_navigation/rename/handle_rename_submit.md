---
okf_version: "0.2"
type: Function
title: handle_rename_submit
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/rename.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/rename/handle_rename_submit
language: rust
---

# handle_rename_submit

## Signature

```rust
impl Oxide { pub(crate) fn handle_rename_submit(&mut self) -> iced::Task<Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 54–149 in `crates/oxide-app/src/app/handlers/dock/project_navigation/rename.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rename](/crates/oxide-app/src/app/handlers/dock/project_navigation/rename.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
