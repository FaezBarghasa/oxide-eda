---
okf_version: "0.2"
type: Function
title: write_first_run_tour_dismissed
description: Persist the dismissal flag without clobbering other keys.
resource: crates/oxide-app/src/fonts/misc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/fonts/misc/write_first_run_tour_dismissed
language: rust
---

# write_first_run_tour_dismissed

Persist the dismissal flag without clobbering other keys.

## Signature

```rust
pub fn write_first_run_tour_dismissed(dismissed: bool)
```

## Visibility

- `pub`

## Docstring

Persist the dismissal flag without clobbering other keys.

## Source
Lines 25–32 in `crates/oxide-app/src/fonts/misc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [misc](/crates/oxide-app/src/fonts/misc.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| called_by | [dispatch_update](/crates/oxide-app/src/app/dispatch/mod/dispatch_update.md) |
| called_by | [dispatch_overlay_message](/crates/oxide-app/src/app/dispatch/overlay/dispatch_overlay_message.md) |
