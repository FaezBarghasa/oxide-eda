---
okf_version: "0.2"
type: Function
title: write_snap_enabled_pref
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/write_snap_enabled_pref
language: rust
---

# write_snap_enabled_pref

## Signature

```rust
pub fn write_snap_enabled_pref(enabled: bool)
```

## Visibility

- `pub`

## Source
Lines 739–741 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [write_snap_enabled_pref_at](/crates/oxide-app/src/fonts/mod/write_snap_enabled_pref_at.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [dispatch_ui_message](/crates/oxide-app/src/app/dispatch/ui/dispatch_ui_message.md) |
