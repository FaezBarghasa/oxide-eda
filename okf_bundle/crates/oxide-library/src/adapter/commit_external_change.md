---
okf_version: "0.2"
type: Function
title: commit_external_change
description: "Stage and commit a file the caller already wrote to disk, using"
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/commit_external_change
language: rust
---

# commit_external_change

Stage and commit a file the caller already wrote to disk, using

## Signature

```rust
fn commit_external_change(
        &self,
        _abs_path: &std::path::Path,
        _message: &str,
    ) -> Result<(), LibraryError>
```

## Docstring

Stage and commit a file the caller already wrote to disk, using
the adapter's own version-control surface. `abs_path` must live
under [`Self::root_path`]; adapters whose `root_path` is `None`
(e.g. the database backend) treat this as a no-op.

Used by the standalone primitive editor tabs (`.snxsym` /
`.snxfpt`): the editor writes the full container at the user's
chosen path (preserving multi-symbol semantics that the
adapter's per-primitive `save_*` would lose), then asks the
adapter to commit so the edit lands in git history. Without
this hook, standalone-tab edits would leave the working tree
permanently dirty until the next `save_*` call paths.

Default impl returns `Ok(())` so non-git backends transparently
skip the commit step.

## Source
Lines 481–487 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
