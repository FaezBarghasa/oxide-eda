---
okf_version: "0.2"
type: Class
title: LocalGitProjectAdapter
description: Project-scoped git adapter.
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/LocalGitProjectAdapter
language: rust
---

# LocalGitProjectAdapter

Project-scoped git adapter.

## Signature

```rust
pub struct LocalGitProjectAdapter
```

## Decorators

- `derive(Debug)`

## Visibility

- `pub`

## Docstring

Project-scoped git adapter.

Construct via [`LocalGitProjectAdapter::open_or_init`]; the
adapter does *not* take ownership of the project file itself —
the app layer continues to write `.snxprj` / `.snxsch` etc.
atomically. The adapter is purely the version-control layer.
[derive(Debug)]

## Methods

- `project_root`
- `git_lock`

## Source
Lines 50–56 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
