---
okf_version: "0.2"
type: Function
title: dispatch_command_palette_message
resource: crates/oxide-app/src/app/dispatch/command_palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/command_palette/dispatch_command_palette_message
language: rust
---

# dispatch_command_palette_message

## Signature

```rust
impl Oxide { pub(super) fn dispatch_command_palette_message(
        &mut self,
        message: CommandPaletteMsg,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Source
Lines 12–40 in `crates/oxide-app/src/app/dispatch/command_palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_palette](/crates/oxide-app/src/app/dispatch/command_palette.md) |
