---
okf_version: "0.2"
type: Function
title: handle_undock_tab
description: "Pop tab `idx` into its own OS window. The tab stays in"
resource: crates/oxide-app/src/app/handlers/erc/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/modals/handle_undock_tab_1
language: rust
---

# handle_undock_tab

Pop tab `idx` into its own OS window. The tab stays in

## Signature

```rust
pub(crate) fn handle_undock_tab(&mut self, idx: usize) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Pop tab `idx` into its own OS window. The tab stays in
`document_state.tabs` so reattach is a pure UI flip — closing the
popped-out window via `SecondaryWindowClosed` just drops the entry
from `ui_state.windows` and the tab re-appears in the tab bar.

## Source
Lines 66–147 in `crates/oxide-app/src/app/handlers/erc/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/handlers/erc/modals.md) |
