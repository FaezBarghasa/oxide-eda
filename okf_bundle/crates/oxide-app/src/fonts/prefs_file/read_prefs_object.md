---
okf_version: "0.2"
type: Function
title: read_prefs_object
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/read_prefs_object
language: rust
---

# read_prefs_object

## Signature

```rust
fn read_prefs_object(path: &Path) -> serde_json::Map<String, serde_json::Value>
```

## Source
Lines 407–415 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| called_by | [absent_prefs_file_is_created](/crates/oxide-app/src/fonts/prefs_file/absent_prefs_file_is_created.md) |
| called_by | [empty_prefs_file_is_treated_as_absent](/crates/oxide-app/src/fonts/prefs_file/empty_prefs_file_is_treated_as_absent.md) |
| called_by | [existing_keys_survive_an_unrelated_write](/crates/oxide-app/src/fonts/prefs_file/existing_keys_survive_an_unrelated_write.md) |
| called_by | [writes_resume_after_the_broken_file_is_moved_aside](/crates/oxide-app/src/fonts/prefs_file/writes_resume_after_the_broken_file_is_moved_aside.md) |
