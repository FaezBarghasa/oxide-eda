---
okf_version: "0.2"
type: Function
title: core_to_message
description: "Resolve a command for DISPATCH, reporting when it cannot be resolved."
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/core_to_message
language: rust
---

# core_to_message

Resolve a command for DISPATCH, reporting when it cannot be resolved.

## Signature

```rust
pub(crate) fn core_to_message(command: &AppCommandId) -> Option<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Resolve a command for DISPATCH, reporting when it cannot be resolved.

Maps a stable command id onto the app's namespaced [`Message`] tree.
Commands without a live dispatch arm return `None`, and
[`log_unmapped`] says so — naming the id and which of the two
failure modes it hit — so an invocation can no longer vanish without
a trace. 64 catalog ids are in that state, every one of them bound to
a trigger in a shipped profile; the set is pinned by
`tests::UNMAPPED_CATALOG_IDS` and may only shrink.

Use [`is_dispatchable`] instead when you are merely *asking* whether a
command resolves. #619: the palette filtered its rows through this
function, so building the palette view emitted one warning per
unmapped id — 2432 records in a single session, against a 200-entry
ring buffer, which evicted every real diagnostic before anyone could
read it. The message was also false on that path: nothing had been
invoked.

## Source
Lines 30–36 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [log_unmapped](/crates/oxide-app/src/app/command/bridge/log_unmapped.md) |
| called_by | [resolve](/crates/oxide-app/src/app/command/bridge/resolve.md) |
| called_by | [the_newly_wired_ids_reach_their_named_messages](/crates/oxide-app/src/app/command/bridge/the_newly_wired_ids_reach_their_named_messages.md) |
| called_by | [dispatch_command](/crates/oxide-app/src/app/command/mod/dispatch_command.md) |
| called_by | [execute_command_palette_selected](/crates/oxide-app/src/app/dispatch/command_palette/execute_command_palette_selected.md) |
