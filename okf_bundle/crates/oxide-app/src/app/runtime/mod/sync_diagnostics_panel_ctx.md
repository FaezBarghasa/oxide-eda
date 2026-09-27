---
okf_version: "0.2"
type: Function
title: sync_diagnostics_panel_ctx
description: Republish the diagnostics ring buffer into the Messages panel.
resource: crates/oxide-app/src/app/runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/mod/sync_diagnostics_panel_ctx
language: rust
---

# sync_diagnostics_panel_ctx

Republish the diagnostics ring buffer into the Messages panel.

## Signature

```rust
impl Oxide { pub(in crate::app) fn sync_diagnostics_panel_ctx(&mut self) }
```

## Visibility

- `pub(in crate::app)`

## Docstring

Republish the diagnostics ring buffer into the Messages panel.

`finish_update` calls this, but not every dispatcher calls
`finish_update` — `dispatch_preferences_message` deliberately does
not, because reloading the History panel and draining git commits
on every keystroke in a modal is not what that dispatcher is for.
Those dispatchers call this directly instead, so a record they
emit is on screen in the same frame rather than whenever some
unrelated later message happens to run `finish_update`.

## Source
Lines 62–66 in `crates/oxide-app/src/app/runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [runtime](/crates/oxide-app/src/app/runtime/mod.md) |
| calls | [configured_level_label](/crates/oxide-app/src/diagnostics/configured_level_label.md) |
| calls | [recent_entries](/crates/oxide-app/src/diagnostics/recent_entries.md) |
