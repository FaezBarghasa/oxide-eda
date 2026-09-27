---
okf_version: "0.2"
type: Function
title: forget_refusal
description: "Forget a path that loaded cleanly, so a file that is repaired and"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/forget_refusal
language: rust
---

# forget_refusal

Forget a path that loaded cleanly, so a file that is repaired and

## Signature

```rust
fn forget_refusal(path: &Path)
```

## Docstring

Forget a path that loaded cleanly, so a file that is repaired and
then broken again reports the second break too.

## Source
Lines 158–160 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [latch](/crates/oxide-app/src/fonts/prefs_file/latch.md) |
| called_by | [move_prefs_file_aside_at](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at.md) |
| called_by | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
