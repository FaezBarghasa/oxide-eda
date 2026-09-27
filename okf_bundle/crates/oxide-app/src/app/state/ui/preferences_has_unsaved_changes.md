---
okf_version: "0.2"
type: Function
title: preferences_has_unsaved_changes
description: True when closing Preferences without Save would lose work — drives
resource: crates/oxide-app/src/app/state/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/state/ui/preferences_has_unsaved_changes
language: rust
---

# preferences_has_unsaved_changes

True when closing Preferences without Save would lose work — drives

## Signature

```rust
impl UiState { pub fn preferences_has_unsaved_changes(&self) -> bool }
```

## Visibility

- `pub`

## Docstring

True when closing Preferences without Save would lose work — drives
the Save/Discard footer + every dirty-close guard. The draft
comparator plus the sticky flag for imperative edits it can't see
(see [`Self::preferences_dirty_sticky`]).

## Source
Lines 424–426 in `crates/oxide-app/src/app/state/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/app/state/ui.md) |
