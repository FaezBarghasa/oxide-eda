---
okf_version: "0.2"
type: Function
title: is_dispatchable
description: Does this command resolve to a message? Silent — no diagnostics.
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/is_dispatchable
language: rust
---

# is_dispatchable

Does this command resolve to a message? Silent — no diagnostics.

## Signature

```rust
pub(crate) fn is_dispatchable(command: &AppCommandId) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

Does this command resolve to a message? Silent — no diagnostics.

The counterpart to [`core_to_message`] for callers that are asking a
question rather than dispatching. A query must not narrate.

## Source
Lines 42–44 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| called_by | [build_catalog](/crates/oxide-app/src/app/command_palette/build_catalog.md) |
| called_by | [every_palette_command_row_resolves_through_the_bridge](/crates/oxide-app/src/app/command_palette/every_palette_command_row_resolves_through_the_bridge.md) |
