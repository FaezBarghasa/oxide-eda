---
okf_version: "0.2"
type: Function
title: begin_or_continue_chord
description: "Attach the pending chord buffer to `target`, dropping whatever"
resource: crates/oxide-app/src/app/dispatch/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/keymap/begin_or_continue_chord_1
language: rust
---

# begin_or_continue_chord

Attach the pending chord buffer to `target`, dropping whatever

## Signature

```rust
fn begin_or_continue_chord(&mut self, target: InputTarget)
```

## Docstring

Attach the pending chord buffer to `target`, dropping whatever
prefix belonged to a different one (#559).

`keymap_pending_sequence` is app-wide, so without this a sequence
could begin in one window, complete in another, and resolve
against the contexts of a third.

## Source
Lines 91–97 in `crates/oxide-app/src/app/dispatch/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/app/dispatch/keymap.md) |
