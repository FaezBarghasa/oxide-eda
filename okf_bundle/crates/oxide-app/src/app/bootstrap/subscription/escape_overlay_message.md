---
okf_version: "0.2"
type: Function
title: escape_overlay_message
description: "Resolve the Esc ladder against live state, or `None` when no"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/escape_overlay_message
language: rust
---

# escape_overlay_message

Resolve the Esc ladder against live state, or `None` when no

## Signature

```rust
impl Oxide { pub(crate) fn escape_overlay_message(&self) -> Option<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Resolve the Esc ladder against live state, or `None` when no
overlay claims the key.

The single entry point for the `Message::EscapePressed` handler
(#535). `OpenOverlays` stays private to this module: the ladder is
one concern in one file, and callers only ever want the answer.

## Source
Lines 380–382 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
