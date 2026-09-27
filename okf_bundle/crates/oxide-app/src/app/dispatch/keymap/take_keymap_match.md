---
okf_version: "0.2"
type: Function
title: take_keymap_match
description: Look the current pending sequence up in the active keymap.
resource: crates/oxide-app/src/app/dispatch/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/keymap/take_keymap_match
language: rust
---

# take_keymap_match

Look the current pending sequence up in the active keymap.

## Signature

```rust
impl Oxide { fn take_keymap_match(&mut self, contexts: &[ShortcutContext]) -> Option<Task<Message>> }
```

## Docstring

Look the current pending sequence up in the active keymap.

Returns `Some(task)` when the sequence is consumed — either it
resolved to a command (dispatched), matched a binding with no
dispatch arm yet (no-op), or is a live prefix of a longer chord
(buffer kept, no-op). Returns `None` on a definite miss so the
caller can apply its restart retry / fall through.

## Source
Lines 64–83 in `crates/oxide-app/src/app/dispatch/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/app/dispatch/keymap.md) |
