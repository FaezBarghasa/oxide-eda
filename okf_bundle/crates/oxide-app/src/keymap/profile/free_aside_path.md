---
okf_version: "0.2"
type: Function
title: free_aside_path
description: First free sibling name to move the live shortcuts file aside to
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/free_aside_path
language: rust
---

# free_aside_path

First free sibling name to move the live shortcuts file aside to

## Signature

```rust
fn free_aside_path(path: &Path) -> Result<PathBuf, std::io::Error>
```

## Docstring

First free sibling name to move the live shortcuts file aside to
before a restore overwrites it.

Slot 1 is the plain `.bak`; the ladder numbers from `.bak.2`, so this
covers exactly [`MAX_ASIDE_SLOTS`] names and the exhaustion message
below stays true. A slot counts as free only when
`symlink_metadata` answers `NotFound`; any other error aborts
carrying that errno rather than laddering past it, because
`Path::exists()` answers `false` for a name it merely cannot stat and
`std::fs::rename` silently replaces its destination — the file that
vanished would be the user's earlier backup.

## Source
Lines 486–512 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| called_by | [restore_profiles_at](/crates/oxide-app/src/keymap/profile/restore_profiles_at.md) |
