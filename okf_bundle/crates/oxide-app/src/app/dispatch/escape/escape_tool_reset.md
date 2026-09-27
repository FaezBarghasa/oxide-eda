---
okf_version: "0.2"
type: Function
title: escape_tool_reset
description: "Cancel the placement session and drop back to `Tool::Select`."
resource: crates/oxide-app/src/app/dispatch/escape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/escape/escape_tool_reset
language: rust
---

# escape_tool_reset

Cancel the placement session and drop back to `Tool::Select`.

## Signature

```rust
impl Oxide { fn escape_tool_reset(&mut self) -> Task<Message> }
```

## Docstring

Cancel the placement session and drop back to `Tool::Select`.

There is exactly ONE placement session app-wide —
`InteractionState::current_tool` and every placement buffer are
single fields, and a tool picked from an undocked window's toolbar
sets those same globals — so this is the same operation whichever
window the Esc came from. Only the *visuals* are per-window, and
`clear_transient_schematic_tool_state` sweeps all of them.

## Source
Lines 186–190 in `crates/oxide-app/src/app/dispatch/escape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [escape](/crates/oxide-app/src/app/dispatch/escape.md) |
| calls | [Tool](/crates/oxide-app/src/app/documents/Tool.md) |
