---
okf_version: "0.2"
type: Function
title: handle_escape_pressed
description: Resolve one Esc against the window it was typed in.
resource: crates/oxide-app/src/app/dispatch/escape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/escape/handle_escape_pressed_1
language: rust
---

# handle_escape_pressed

Resolve one Esc against the window it was typed in.

## Signature

```rust
pub(crate) fn handle_escape_pressed(
        &mut self,
        window: Option<iced::window::Id>,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Resolve one Esc against the window it was typed in.

## Source
Lines 32–111 in `crates/oxide-app/src/app/dispatch/escape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [escape](/crates/oxide-app/src/app/dispatch/escape.md) |
