---
okf_version: "0.2"
type: Function
title: mount_dependencies
description: "Mount resolved dependencies into `ProjectData` and collect search paths."
resource: crates/oxide-library/src/dependency/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:52Z"
concept_id: crates/oxide-library/src/dependency/manager/mount_dependencies
language: rust
---

# mount_dependencies

Mount resolved dependencies into `ProjectData` and collect search paths.

## Signature

```rust
impl GitDependencyManager { pub fn mount_dependencies(
        &self,
        project_root: &Path,
        lockfile: &ProjectLockfile,
        project_data: &mut ProjectData,
    ) -> Result<MountReport, DependencyError> }
```

## Visibility

- `pub`

## Docstring

Mount resolved dependencies into `ProjectData` and collect search paths.

## Source
Lines 176–211 in `crates/oxide-library/src/dependency/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-library/src/dependency/manager.md) |
