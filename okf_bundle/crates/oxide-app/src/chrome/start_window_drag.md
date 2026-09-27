---
okf_version: "0.2"
type: Function
title: start_window_drag
description: "Begin an OS-level move on the given window. Uses iced's"
resource: crates/oxide-app/src/chrome.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/chrome/start_window_drag
language: rust
---

# start_window_drag

Begin an OS-level move on the given window. Uses iced's

## Signature

```rust
pub fn start_window_drag(id: Id) -> Task<M>
```

## Type Parameters

- `M: 'static + Send`

## Visibility

- `pub`

## Docstring

Begin an OS-level move on the given window. Uses iced's
`window::drag` (winit's `drag_window`) on all platforms.

Historical note: an earlier `WM_SYSCOMMAND SC_MOVE` Win32 detour
was added (`c682d7ca`) on the assumption that winit's
`PostMessageW(WM_NCLBUTTONDOWN)` silently no-ops on borderless
Windows. Verified empirically: iced's `window::drag` actually
works on `decorations: false` windows on the current pinned
winit version. Reverted (2026-05-02) — the SC_MOVE/SC_SIZE
path entered Windows' modal sizing loop, starving iced's
runtime and producing the resize-stretch regression bisected
to that commit (see `docs/internal/TEST_CHECKLIST_v0.10_v0.11.md`
F9).

## Source
Lines 52–54 in `crates/oxide-app/src/chrome.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chrome](/crates/oxide-app/src/chrome.md) |
| called_by | [dispatch_window_message](/crates/oxide-app/src/app/dispatch/mod/dispatch_window_message.md) |
| called_by | [handle_start_detached_window_drag](/crates/oxide-app/src/app/handlers/erc/modals/handle_start_detached_window_drag.md) |
