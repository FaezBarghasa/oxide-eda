---
okf_version: "0.2"
type: Class
title: Inner
resource: crates/oxide-library-server/src/locks.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library-server/src/locks/Inner
language: rust
---

# Inner

## Signature

```rust
struct Inner
```

## Methods

- `locks`
- `idle_ttl`

## Source
Lines 74–77 in `crates/oxide-library-server/src/locks.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [locks](/crates/oxide-library-server/src/locks.md) |
| called_by | [claim_keymap_recorder](/crates/oxide-app/src/app/dispatch/input/claim_keymap_recorder.md) |
| called_by | [handle_keymap_pref_message](/crates/oxide-app/src/app/handlers/preferences/keymap/handle_keymap_pref_message.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
| called_by | [view_preferences_body](/crates/oxide-app/src/app/view/chrome/view_preferences_body.md) |
| called_by | [preferences_overlay](/crates/oxide-app/src/app/view/overlays/modals/preferences_overlay.md) |
| called_by | [keymap_rebind_dirty_flag_survives_an_unrelated_appearance_toggle](/crates/oxide-app/tests/regression/preferences_dirty_guard/keymap_rebind_dirty_flag_survives_an_unrelated_appearance_toggle.md) |
| called_by | [theme_import_dirty_flag_survives_an_unrelated_appearance_toggle](/crates/oxide-app/tests/regression/preferences_dirty_guard/theme_import_dirty_flag_survives_an_unrelated_appearance_toggle.md) |
| called_by | [inner](/crates/oxide-app/tests/regression/preferences_export_status/inner.md) |
| called_by | [inner](/crates/oxide-app/tests/regression/preferences_prefs_recovery/inner.md) |
| called_by | [inner](/crates/oxide-app/tests/regression/preferences_symbol_drafts/inner.md) |
| called_by | [inner](/crates/oxide-app/tests/regression/render_config_grid_style/inner.md) |
