---
okf_version: "0.2"
type: Function
title: non_object_root_is_left_alone_and_does_not_panic
description: "A non-object root used to reach the `library_browser_searches`"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/non_object_root_is_left_alone_and_does_not_panic
language: rust
---

# non_object_root_is_left_alone_and_does_not_panic

A non-object root used to reach the `library_browser_searches`

## Signature

```rust
fn non_object_root_is_left_alone_and_does_not_panic()
```

## Decorators

- `test`

## Docstring

A non-object root used to reach the `library_browser_searches`
writer's `.as_object_mut().expect(…)` and panic the UI thread.
[test]

## Source
Lines 509–533 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
