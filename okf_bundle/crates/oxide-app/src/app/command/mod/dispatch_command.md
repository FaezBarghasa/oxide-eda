---
okf_version: "0.2"
type: Function
title: dispatch_command
description: "The Command Registry's dispatch entry point — resolve a stable"
resource: crates/oxide-app/src/app/command/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/command/mod/dispatch_command
language: rust
---

# dispatch_command

The Command Registry's dispatch entry point — resolve a stable

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_command(
        &mut self,
        command: &crate::keymap::AppCommandId,
        _args: CommandArgs,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

The Command Registry's dispatch entry point — resolve a stable
[`crate::keymap::AppCommandId`] and run it exactly as if the
resolved [`Message`] had been sent directly.

Deliberately takes an id + args, never a [`Message`] — the whole
point of the registry is that a caller (keyboard today; menu,
command palette, and a future CLI later) names *what* to run
without knowing the app's internal message shape.

`args` is accepted but unused: [`core_to_message`] ignores it,
because every catalog command is nullary today. It is here only
so this signature is stable ahead of a consumer that needs it —
do not thread it through `core_to_message` speculatively.

An id can fail to resolve two ways, and both are a silent no-op,
never a panic: it isn't in the catalog at all, or it's a real
catalog id with no live arm in `core_to_message` yet. Ids come
from user-editable TOML keymap profiles today and a CLI later —
both are untrusted input, not a programmer error.

## Source
Lines 38–47 in `crates/oxide-app/src/app/command/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command](/crates/oxide-app/src/app/command/mod.md) |
| calls | [core_to_message](/crates/oxide-app/src/app/command/bridge/core_to_message.md) |
