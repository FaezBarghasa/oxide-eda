---
okf_version: "0.2"
type: Function
title: move_prefs_file_aside
description: "[`move_prefs_file_aside_at`] against the resolved user prefs path."
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside
language: rust
---

# move_prefs_file_aside

[`move_prefs_file_aside_at`] against the resolved user prefs path.

## Signature

```rust
pub fn move_prefs_file_aside() -> Result<Option<PathBuf>, std::io::Error>
```

## Visibility

- `pub`

## Docstring

[`move_prefs_file_aside_at`] against the resolved user prefs path.

## Source
Lines 386–388 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [move_prefs_file_aside_at](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at.md) |
| calls | [prefs_file_path](/crates/oxide-app/src/fonts/prefs_file/prefs_file_path.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
