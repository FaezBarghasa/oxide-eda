---
okf_version: "0.2"
type: Class
title: PendingGitCommit
description: v0.23 — One queued commit for the async git pipeline. Stays
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/PendingGitCommit
language: rust
---

# PendingGitCommit

v0.23 — One queued commit for the async git pipeline. Stays

## Signature

```rust
pub struct PendingGitCommit
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

v0.23 — One queued commit for the async git pipeline. Stays
resident in `DocumentState.pending_git_commits` until
`finish_update` drains it into a `Task::perform`.
[derive(Debug, Clone)]

## Methods

- `project_root`
- `rel_path`
- `message`

## Source
Lines 450–454 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
