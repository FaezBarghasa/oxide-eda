---
okf_version: "0.2"
type: Function
title: writes_resume_after_the_broken_file_is_moved_aside
description: "The end-to-end shape of #602: a refused write, the recovery, and"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/writes_resume_after_the_broken_file_is_moved_aside
language: rust
---

# writes_resume_after_the_broken_file_is_moved_aside

The end-to-end shape of #602: a refused write, the recovery, and

## Signature

```rust
fn writes_resume_after_the_broken_file_is_moved_aside()
```

## Decorators

- `test`

## Docstring

The end-to-end shape of #602: a refused write, the recovery, and
a write that lands again — with the user's original still on disk.
[test]

## Source
Lines 894–929 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| calls | [move_prefs_file_aside_at](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at.md) |
| calls | [read_prefs_object](/crates/oxide-app/src/fonts/prefs_file/read_prefs_object.md) |
