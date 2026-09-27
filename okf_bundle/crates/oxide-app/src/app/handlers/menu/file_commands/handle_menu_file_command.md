---
okf_version: "0.2"
type: Function
title: handle_menu_file_command
resource: crates/oxide-app/src/app/handlers/menu/file_commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/file_commands/handle_menu_file_command
language: rust
---

# handle_menu_file_command

## Signature

```rust
impl Oxide { pub(super) fn handle_menu_file_command(&mut self, msg: &MenuMessage) -> Option<Task<Message>> }
```

## Visibility

- `pub(super)`

## Source
Lines 6–135 in `crates/oxide-app/src/app/handlers/menu/file_commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [file_commands](/crates/oxide-app/src/app/handlers/menu/file_commands.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
