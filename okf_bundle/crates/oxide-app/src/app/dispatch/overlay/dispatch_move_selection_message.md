---
okf_version: "0.2"
type: Function
title: dispatch_move_selection_message
description: "Move Selection dialog family handler (namespaced, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/overlay/dispatch_move_selection_message
language: rust
---

# dispatch_move_selection_message

Move Selection dialog family handler (namespaced, ADR-0001 D3).

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_move_selection_message(
        &mut self,
        msg: MoveSelectionMsg,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Move Selection dialog family handler (namespaced, ADR-0001 D3).
Altium numeric ΔX / ΔY move on the current selection.

## Source
Lines 97–117 in `crates/oxide-app/src/app/dispatch/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/app/dispatch/overlay.md) |
