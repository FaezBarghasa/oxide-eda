---
okf_version: "0.2"
type: Function
title: recover_init
description: "Recover a library whose `.git/` directory was deleted"
resource: crates/oxide-library/src/adapters/local_git/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/mod/recover_init
language: rust
---

# recover_init

Recover a library whose `.git/` directory was deleted

## Signature

```rust
impl LocalGitAdapter { pub fn recover_init(file_path: impl AsRef<Path>) -> Result<Self, LibraryError> }
```

## Visibility

- `pub`

## Docstring

Recover a library whose `.git/` directory was deleted
out-from-under-it. Re-runs `git init` at the parent directory
and stages the current working tree as a fresh
"snxlib re-init" commit. Past history is lost — the recovery
dialog (Stage 10) is responsible for warning the user before
landing here.

## Source
Lines 242–309 in `crates/oxide-library/src/adapters/local_git/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git](/crates/oxide-library/src/adapters/local_git/mod.md) |
| calls | [parent_dir](/crates/oxide-library/src/adapters/local_git/helpers/parent_dir.md) |
| calls | [synthesize_manifest](/crates/oxide-library/src/adapters/local_git/helpers/synthesize_manifest.md) |
