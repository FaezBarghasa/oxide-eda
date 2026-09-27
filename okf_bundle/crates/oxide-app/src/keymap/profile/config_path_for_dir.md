---
okf_version: "0.2"
type: Function
title: config_path_for_dir
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/config_path_for_dir
language: rust
---

# config_path_for_dir

## Signature

```rust
pub fn config_path_for_dir(base: &Path) -> PathBuf
```

## Visibility

- `pub`

## Source
Lines 358–360 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [config_root_for_dir](/crates/oxide-app/src/config_root/config_root_for_dir.md) |
