---
okf_version: "0.2"
type: Function
title: lockfile_path
description: "Path to `project.lock` for a given project directory."
resource: crates/oxide-library/src/dependency/lockfile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:28:04Z"
concept_id: crates/oxide-library/src/dependency/lockfile/lockfile_path
language: rust
---

# lockfile_path

Path to `project.lock` for a given project directory.

## Signature

```rust
impl LockfileManager { pub fn lockfile_path(project_root: &Path) -> PathBuf }
```

## Visibility

- `pub`

## Docstring

Path to `project.lock` for a given project directory.

## Source
Lines 20–22 in `crates/oxide-library/src/dependency/lockfile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lockfile](/crates/oxide-library/src/dependency/lockfile.md) |
