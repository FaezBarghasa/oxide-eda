---
okf_version: "0.2"
type: Function
title: differs_from
description: True when this working copy has diverged from the live profile set —
resource: crates/oxide-app/src/keymap/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/editor/differs_from_1
language: rust
---

# differs_from

True when this working copy has diverged from the live profile set —

## Signature

```rust
pub fn differs_from(&self, live: &ShortcutProfileSet) -> bool
```

## Visibility

- `pub`

## Docstring

True when this working copy has diverged from the live profile set —
feeds the Preferences dirty comparator. Valid trigger edits are
applied straight into `profiles` (so the set comparison sees them,
including an edit retyped back to the original, which compares
clean); an *invalid* draft counts as dirty on its own because it is
pending user input that Save refuses to commit.

## Source
Lines 192–194 in `crates/oxide-app/src/keymap/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/keymap/editor.md) |
