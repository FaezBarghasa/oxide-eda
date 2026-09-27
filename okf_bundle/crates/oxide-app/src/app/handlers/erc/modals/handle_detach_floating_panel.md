---
okf_version: "0.2"
type: Function
title: handle_detach_floating_panel
description: "Remove the floating panel at `idx` and open an OS window that"
resource: crates/oxide-app/src/app/handlers/erc/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/modals/handle_detach_floating_panel
language: rust
---

# handle_detach_floating_panel

Remove the floating panel at `idx` and open an OS window that

## Signature

```rust
impl Oxide { pub(crate) fn handle_detach_floating_panel(&mut self, idx: usize) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Remove the floating panel at `idx` and open an OS window that
renders that panel's content. Closing the OS window re-docks the
panel to the right column — see `SecondaryWindowClosed` in
dispatch/mod.rs.

## Source
Lines 153–177 in `crates/oxide-app/src/app/handlers/erc/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/handlers/erc/modals.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
