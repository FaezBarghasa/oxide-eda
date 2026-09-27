---
okf_version: "0.2"
type: Function
title: handle_key_input
description: "The single entry point for `Message::KeyInput`."
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/handle_key_input
language: rust
---

# handle_key_input

The single entry point for `Message::KeyInput`.

## Signature

```rust
impl Oxide { pub(crate) fn handle_key_input(
        &mut self,
        window: iced::window::Id,
        event: keyboard::Event,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

The single entry point for `Message::KeyInput`.

## Source
Lines 191–200 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
