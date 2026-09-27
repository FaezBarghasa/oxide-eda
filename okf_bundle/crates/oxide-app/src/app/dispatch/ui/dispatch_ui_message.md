---
okf_version: "0.2"
type: Function
title: dispatch_ui_message
resource: crates/oxide-app/src/app/dispatch/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/ui/dispatch_ui_message
language: rust
---

# dispatch_ui_message

## Signature

```rust
impl Oxide { pub(super) fn dispatch_ui_message(&mut self, message: UiMsg) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Source
Lines 6–142 in `crates/oxide-app/src/app/dispatch/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/app/dispatch/ui.md) |
| calls | [write_theme_pref](/crates/oxide-app/src/fonts/mod/write_theme_pref.md) |
| calls | [write_grid_visible_pref](/crates/oxide-app/src/fonts/mod/write_grid_visible_pref.md) |
| calls | [write_snap_enabled_pref](/crates/oxide-app/src/fonts/mod/write_snap_enabled_pref.md) |
