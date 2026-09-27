---
okf_version: "0.2"
type: Function
title: restore_from_lockfile
description: "Restore exact dependencies strictly from `project.lock`."
resource: crates/oxide-library/src/dependency/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:52Z"
concept_id: crates/oxide-library/src/dependency/manager/restore_from_lockfile_1
language: rust
---

# restore_from_lockfile

Restore exact dependencies strictly from `project.lock`.

## Signature

```rust
pub fn restore_from_lockfile(
        &self,
        project_root: &Path,
        lockfile: &ProjectLockfile,
    ) -> Result<(), DependencyError>
```

## Visibility

- `pub`

## Docstring

Restore exact dependencies strictly from `project.lock`.

## Source
Lines 114–148 in `crates/oxide-library/src/dependency/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-library/src/dependency/manager.md) |
