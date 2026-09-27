---
okf_version: "0.2"
type: Function
title: config_root_for_dir
description: "Join the `oxide` subdirectory onto an arbitrary base directory."
resource: crates/oxide-app/src/config_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/config_root/config_root_for_dir
language: rust
---

# config_root_for_dir

Join the `oxide` subdirectory onto an arbitrary base directory.

## Signature

```rust
pub fn config_root_for_dir(base: &std::path::Path) -> PathBuf
```

## Visibility

- `pub`

## Docstring

Join the `oxide` subdirectory onto an arbitrary base directory.

[`config_root`] uses this for the real OS config dir, and it's the
one place the two `config_path_for_dir` test helpers
(`keymap::profile`, `library::settings::persistence`) get the same
join from — before #440 hoisted this, production went through
`config_root()` while those two test helpers still hardcoded
`base.join("oxide")` themselves, so a rename of the folder here
would have silently diverged production from the tests that are
supposed to prove it (#440 review).

## Source
Lines 66–68 in `crates/oxide-app/src/config_root.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config_root](/crates/oxide-app/src/config_root.md) |
| called_by | [config_root](/crates/oxide-app/src/config_root/config_root.md) |
| called_by | [config_path_for_dir](/crates/oxide-app/src/keymap/profile/config_path_for_dir.md) |
| called_by | [config_path_for_dir](/crates/oxide-app/src/library/settings/persistence/config_path_for_dir.md) |
