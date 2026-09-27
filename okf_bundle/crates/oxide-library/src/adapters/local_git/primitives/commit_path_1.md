---
okf_version: "0.2"
type: Function
title: commit_path
description: "Stage `rel_path` and create a new commit. Used by primitive saves"
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/commit_path_1
language: rust
---

# commit_path

Stage `rel_path` and create a new commit. Used by primitive saves

## Signature

```rust
pub(super) fn commit_path(
        &self,
        rel_path: &str,
        message: &str,
        fallback_message: &str,
    ) -> Result<(), LibraryError>
```

## Visibility

- `pub(super)`

## Docstring

Stage `rel_path` and create a new commit. Used by primitive saves
(`*.snx*` files) and table writes (the `.snxlib` itself). When
the parent directory has no `.git/`, this is a no-op — the file
has already been written to disk by the caller, and the user
opted out of version control at create time. They can opt in
later via the (forthcoming) Enable Version Control flow.

## Source
Lines 182–240 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
| calls | [add_path](/crates/oxide-app/src/panels/components_panel/global_prefs/add_path.md) |
