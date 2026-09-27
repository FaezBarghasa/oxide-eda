---
okf_version: "0.2"
type: Function
title: write_lockfile
description: "Write `project.lock` atomically."
resource: crates/oxide-library/src/dependency/lockfile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:28:04Z"
concept_id: crates/oxide-library/src/dependency/lockfile/write_lockfile
language: rust
---

# write_lockfile

Write `project.lock` atomically.

## Signature

```rust
impl LockfileManager { pub fn write_lockfile(
        project_root: &Path,
        lockfile: &ProjectLockfile,
    ) -> Result<(), DependencyError> }
```

## Visibility

- `pub`

## Docstring

Write `project.lock` atomically.

## Source
Lines 44–61 in `crates/oxide-library/src/dependency/lockfile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lockfile](/crates/oxide-library/src/dependency/lockfile.md) |
