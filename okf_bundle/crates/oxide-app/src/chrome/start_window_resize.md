---
okf_version: "0.2"
type: Function
title: start_window_resize
description: Begin an OS-level resize on the given window in the direction the
resource: crates/oxide-app/src/chrome.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/chrome/start_window_resize
language: rust
---

# start_window_resize

Begin an OS-level resize on the given window in the direction the

## Signature

```rust
pub fn start_window_resize(id: Id, direction: Direction) -> Task<M>
```

## Type Parameters

- `M: 'static + Send`

## Visibility

- `pub`

## Docstring

Begin an OS-level resize on the given window in the direction the
user grabbed (one of the eight cardinal / corner edges). Uses
iced's `window::drag_resize` (winit's `drag_resize_window`) on
all platforms. See `start_window_drag` for the rationale behind
dropping the Win32 SC_SIZE detour.

## Source
Lines 61–63 in `crates/oxide-app/src/chrome.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chrome](/crates/oxide-app/src/chrome.md) |
| called_by | [dispatch_window_message](/crates/oxide-app/src/app/dispatch/mod/dispatch_window_message.md) |
