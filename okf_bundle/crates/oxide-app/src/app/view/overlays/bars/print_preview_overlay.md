---
okf_version: "0.2"
type: Function
title: print_preview_overlay
description: "Print preview overlay — Altium parity: opens as a separate OS"
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/print_preview_overlay
language: rust
---

# print_preview_overlay

Print preview overlay — Altium parity: opens as a separate OS

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn print_preview_overlay(&self) -> Option<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Print preview overlay — Altium parity: opens as a separate OS
window (see `handle_print_preview_requested → handle_detach_modal`)
so it can be dragged outside the app's client area. Only fall
back to the in-window overlay if the OS window failed to open.

## Source
Lines 87–94 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
