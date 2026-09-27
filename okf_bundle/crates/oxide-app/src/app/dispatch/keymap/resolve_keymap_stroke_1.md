---
okf_version: "0.2"
type: Function
title: resolve_keymap_stroke
description: "Resolve one forwarded keystroke against the active keymap,"
resource: crates/oxide-app/src/app/dispatch/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/keymap/resolve_keymap_stroke_1
language: rust
---

# resolve_keymap_stroke

Resolve one forwarded keystroke against the active keymap,

## Signature

```rust
pub(super) fn resolve_keymap_stroke(
        &mut self,
        window: Option<iced::window::Id>,
        stroke: KeyStroke,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Resolve one forwarded keystroke against the active keymap,
accumulating multi-stroke chords in `keymap_pending_sequence`.

A resolved command is dispatched through the normal
[`Oxide::dispatch_update`] path, so it behaves exactly as if the
mapped message had been sent directly. A partial chord keeps the
buffer and waits; a definite miss clears it (with a single-stroke
restart retry so a stale prefix can't wedge later keys).

## Source
Lines 29–55 in `crates/oxide-app/src/app/dispatch/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/app/dispatch/keymap.md) |
