---
okf_version: "0.2"
type: Function
title: seed_preferences_drafts_from_live
description: Seed every Preferences draft from its saved live value and reset the
resource: crates/oxide-app/src/app/handlers/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/preferences/mod/seed_preferences_drafts_from_live
language: rust
---

# seed_preferences_drafts_from_live

Seed every Preferences draft from its saved live value and reset the

## Signature

```rust
impl Oxide { fn seed_preferences_drafts_from_live(&mut self) }
```

## Docstring

Seed every Preferences draft from its saved live value and reset the
dialog's transient keymap state + dirty flags. Shared by open (fresh
dialog) and [`Self::revert_preferences_drafts`] (discard) so a future
draft field can't be added to one block and silently missed by the
other — the drift class `preferences_draft_differs` exists to repair.

## Source
Lines 53–94 in `crates/oxide-app/src/app/handlers/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/app/handlers/preferences/mod.md) |
