---
okf_version: "0.2"
type: Function
title: custom_theme_json
resource: crates/oxide-app/tests/regression/preferences_dirty_guard.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_dirty_guard/custom_theme_json
language: rust
---

# custom_theme_json

## Signature

```rust
fn custom_theme_json(name: &str) -> String
```

## Source
Lines 12–19 in `crates/oxide-app/tests/regression/preferences_dirty_guard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_dirty_guard](/crates/oxide-app/tests/regression/preferences_dirty_guard.md) |
| calls | [theme_tokens](/crates/oxide-types/src/theme/theme_tokens.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
| called_by | [theme_import_dirty_flag_survives_an_unrelated_appearance_toggle](/crates/oxide-app/tests/regression/preferences_dirty_guard/theme_import_dirty_flag_survives_an_unrelated_appearance_toggle.md) |
