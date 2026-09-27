---
okf_version: "0.2"
type: Function
title: revert_preferences_drafts
description: Revert every Preferences live-preview draft back to its saved value and
resource: crates/oxide-app/src/app/handlers/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/preferences/mod/revert_preferences_drafts_1
language: rust
---

# revert_preferences_drafts

Revert every Preferences live-preview draft back to its saved value and

## Signature

```rust
pub(crate) fn revert_preferences_drafts(&mut self)
```

## Visibility

- `pub(crate)`

## Docstring

Revert every Preferences live-preview draft back to its saved value and
repaint, so discarding — the Discard & Close button, or the backstop
for a dirty window destroyed outside the guarded close paths — drops
unsaved previews instead of leaving them silently active (e.g. the
experimental PCB GPU render toggle would otherwise keep rendering on
the GPU with the checkbox showing unchecked). Does not touch
`preferences_open` or close the window; callers own that. Idempotent —
safe to call more than once.

## Source
Lines 147–180 in `crates/oxide-app/src/app/handlers/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/app/handlers/preferences/mod.md) |
| calls | [theme_tokens](/crates/oxide-types/src/theme/theme_tokens.md) |
