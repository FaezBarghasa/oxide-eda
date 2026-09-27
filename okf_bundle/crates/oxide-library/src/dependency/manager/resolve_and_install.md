---
okf_version: "0.2"
type: Function
title: resolve_and_install
description: "Resolve and update all dependencies declared in `project_data`,"
resource: crates/oxide-library/src/dependency/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:52Z"
concept_id: crates/oxide-library/src/dependency/manager/resolve_and_install
language: rust
---

# resolve_and_install

Resolve and update all dependencies declared in `project_data`,

## Signature

```rust
impl GitDependencyManager { pub fn resolve_and_install(
        &self,
        project_root: &Path,
        project_data: &ProjectData,
    ) -> Result<ProjectLockfile, DependencyError> }
```

## Visibility

- `pub`

## Docstring

Resolve and update all dependencies declared in `project_data`,
cloning/updating them into `.oxide/deps/` and writing `project.lock`.

## Source
Lines 64–111 in `crates/oxide-library/src/dependency/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-library/src/dependency/manager.md) |
