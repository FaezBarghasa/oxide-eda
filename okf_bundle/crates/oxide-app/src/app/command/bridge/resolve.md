---
okf_version: "0.2"
type: Function
title: resolve
description: "The resolution itself. Pure: no logging, no side effects, so both"
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/resolve
language: rust
---

# resolve

The resolution itself. Pure: no logging, no side effects, so both

## Signature

```rust
fn resolve(command: &AppCommandId) -> Option<Message>
```

## Docstring

The resolution itself. Pure: no logging, no side effects, so both
entry points above are built from one table and cannot drift.

## Source
Lines 48–155 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [CanvasEvent](/crates/oxide-app/src/canvas/mod/CanvasEvent.md) |
| calls | [Tool](/crates/oxide-app/src/app/documents/Tool.md) |
| calls | [action_for_id](/crates/oxide-app/src/app/command/active_bar/action_for_id.md) |
