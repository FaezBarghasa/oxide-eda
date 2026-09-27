---
okf_version: "0.2"
type: Function
title: unrelated_panel_message_falls_through_as_none
description: "A message this handler doesn't own must fall through as `None`"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/mod/unrelated_panel_message_falls_through_as_none
language: rust
---

# unrelated_panel_message_falls_through_as_none

A message this handler doesn't own must fall through as `None`

## Signature

```rust
fn unrelated_panel_message_falls_through_as_none()
```

## Decorators

- `test`

## Docstring

A message this handler doesn't own must fall through as `None`
so the dock dispatcher tries the next handler in the chain
(`handlers/dock/mod.rs`), rather than swallowing it.
[test]

## Source
Lines 324–331 in `crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_navigation](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod.md) |
