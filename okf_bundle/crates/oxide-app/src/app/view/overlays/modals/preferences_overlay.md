---
okf_version: "0.2"
type: Function
title: preferences_overlay
description: "Preferences renders inline only if it hasn't been detached into"
resource: crates/oxide-app/src/app/view/overlays/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/modals/preferences_overlay
language: rust
---

# preferences_overlay

Preferences renders inline only if it hasn't been detached into

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn preferences_overlay(&self) -> Option<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Preferences renders inline only if it hasn't been detached into
its own OS window. Open-flow auto-detaches via
`handle_preferences_open_requested`, so this in-window path is
the fallback when the detach failed.

## Source
Lines 20–59 in `crates/oxide-app/src/app/view/overlays/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/overlays/modals.md) |
| calls | [Inner](/crates/oxide-library-server/src/locks/Inner.md) |
