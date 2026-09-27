---
okf_version: "0.2"
type: Function
title: read_lockfile
description: "Read `project.lock` if it exists."
resource: crates/oxide-library/src/dependency/lockfile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:28:04Z"
concept_id: crates/oxide-library/src/dependency/lockfile/read_lockfile
language: rust
---

# read_lockfile

Read `project.lock` if it exists.

## Signature

```rust
impl LockfileManager { pub fn read_lockfile(project_root: &Path) -> Result<Option<ProjectLockfile>, DependencyError> }
```

## Visibility

- `pub`

## Docstring

Read `project.lock` if it exists.

## Source
Lines 25–41 in `crates/oxide-library/src/dependency/lockfile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lockfile](/crates/oxide-library/src/dependency/lockfile.md) |
