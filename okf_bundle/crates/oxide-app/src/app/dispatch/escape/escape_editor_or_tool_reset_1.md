---
okf_version: "0.2"
type: Function
title: escape_editor_or_tool_reset
description: "v0.15 — the main window's post-ladder Esc: if its active tab is a"
resource: crates/oxide-app/src/app/dispatch/escape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/escape/escape_editor_or_tool_reset_1
language: rust
---

# escape_editor_or_tool_reset

v0.15 — the main window's post-ladder Esc: if its active tab is a

## Signature

```rust
fn escape_editor_or_tool_reset(&mut self) -> Task<Message>
```

## Docstring

v0.15 — the main window's post-ladder Esc: if its active tab is a
primitive editor, cancel that editor's state; otherwise fall back
to the schematic tool reset.

## Source
Lines 116–176 in `crates/oxide-app/src/app/dispatch/escape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [escape](/crates/oxide-app/src/app/dispatch/escape.md) |
