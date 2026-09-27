---
okf_version: "0.2"
type: Function
title: project_root
description: Project root that this adapter manages.
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/project_root
language: rust
---

# project_root

Project root that this adapter manages.

## Signature

```rust
impl LocalGitProjectAdapter { pub fn project_root(&self) -> &Path }
```

## Visibility

- `pub`

## Docstring

Project root that this adapter manages.

## Source
Lines 303–305 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
