---
okf_version: "0.2"
type: Function
title: handle_preferences_close_requested
description: "App-level close request (Esc ladder, `PreferencesMsg::Close`). Same"
resource: crates/oxide-app/src/app/handlers/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_close_requested
language: rust
---

# handle_preferences_close_requested

App-level close request (Esc ladder, `PreferencesMsg::Close`). Same

## Signature

```rust
impl Oxide { pub(crate) fn handle_preferences_close_requested(&mut self) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

App-level close request (Esc ladder, `PreferencesMsg::Close`). Same
dirty-close guard as the in-dialog X button (`PrefMsg::Close`): while
unsaved changes exist the dialog stays open — the footer's
"Unsaved changes" bar with Save / Discard & Close is the only way to
resolve them, so no dismiss route can silently drop work. The guard
also covers the detach-failed in-window fallback, where no
`SecondaryWindowClosed` backstop ever fires to revert a lingering
live preview (e.g. the experimental PCB GPU render toggle).

## Source
Lines 39–46 in `crates/oxide-app/src/app/handlers/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/app/handlers/preferences/mod.md) |
