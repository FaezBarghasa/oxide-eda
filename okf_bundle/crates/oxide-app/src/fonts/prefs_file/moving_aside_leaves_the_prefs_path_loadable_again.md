---
okf_version: "0.2"
type: Function
title: moving_aside_leaves_the_prefs_path_loadable_again
description: "The invariant is not \"the path is empty\" — that was the shape, and"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/moving_aside_leaves_the_prefs_path_loadable_again
language: rust
---

# moving_aside_leaves_the_prefs_path_loadable_again

The invariant is not "the path is empty" — that was the shape, and

## Signature

```rust
fn moving_aside_leaves_the_prefs_path_loadable_again()
```

## Decorators

- `test`

## Docstring

The invariant is not "the path is empty" — that was the shape, and
leaving it empty is what let the legacy migration undo the reset.
It is that the broken bytes are off the prefs path and whatever
replaces them loads clean, which is what makes the next write land.
[test]

## Source
Lines 698–723 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [move_prefs_file_aside_at](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at.md) |
