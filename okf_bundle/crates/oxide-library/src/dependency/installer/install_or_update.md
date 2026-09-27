---
okf_version: "0.2"
type: Function
title: install_or_update
description: "Clone or open repository at destination, fetch remotes, and checkout the exact commit."
resource: crates/oxide-library/src/dependency/installer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:18Z"
concept_id: crates/oxide-library/src/dependency/installer/install_or_update
language: rust
---

# install_or_update

Clone or open repository at destination, fetch remotes, and checkout the exact commit.

## Signature

```rust
impl GitInstaller { pub fn install_or_update(
        &self,
        project_root: &Path,
        dep: &ProjectDependency,
        resolved: &ResolvedRef,
    ) -> Result<LockedDependency, DependencyError> }
```

## Visibility

- `pub`

## Docstring

Clone or open repository at destination, fetch remotes, and checkout the exact commit.

## Source
Lines 23–75 in `crates/oxide-library/src/dependency/installer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [installer](/crates/oxide-library/src/dependency/installer.md) |
