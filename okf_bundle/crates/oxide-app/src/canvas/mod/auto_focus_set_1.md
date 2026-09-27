---
okf_version: "0.2"
type: Function
title: auto_focus_set
description: "Compute the \"focus\" uuid set when auto_focus is on — members of"
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/auto_focus_set_1
language: rust
---

# auto_focus_set

Compute the "focus" uuid set when auto_focus is on — members of

## Signature

```rust
fn auto_focus_set(&self) -> Option<std::collections::HashSet<uuid::Uuid>>
```

## Docstring

Compute the "focus" uuid set when auto_focus is on — members of
the current selection. Returns None when auto_focus is off; the
renderer then draws every item at full alpha.

#631 — lives on the borrowing view, not the slot: `auto_focus` is
a `UiState` setting read through `prefs`, while `selected` is the
window's own state reached through `Deref`.

## Source
Lines 338–347 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
