---
okf_version: "0.2"
type: Function
title: claim_keymap_recorder
description: Chord recorder (Preferences ▸ Keyboard Shortcuts). Exclusive while
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/claim_keymap_recorder_1
language: rust
---

# claim_keymap_recorder

Chord recorder (Preferences ▸ Keyboard Shortcuts). Exclusive while

## Signature

```rust
fn claim_keymap_recorder(&self, target: InputTarget, event: &keyboard::Event) -> Claim
```

## Docstring

Chord recorder (Preferences ▸ Keyboard Shortcuts). Exclusive while
open: every raw stroke belongs to the binding under edit and must
NOT reach the live keymap resolver, or recording a shortcut would
also fire it. The pending chord buffer is left untouched — it is
only advanced by the resolver, which we skip.

Held modifiers are claimed too: they drive the live "Ctrl+…" hint
before a key lands.

## Source
Lines 249–281 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
| calls | [Inner](/crates/oxide-library-server/src/locks/Inner.md) |
