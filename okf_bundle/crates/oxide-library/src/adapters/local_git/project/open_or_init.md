---
okf_version: "0.2"
type: Function
title: open_or_init
description: "Open the git repo at `project_root`, or `git init` if no"
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/open_or_init
language: rust
---

# open_or_init

Open the git repo at `project_root`, or `git init` if no

## Signature

```rust
impl LocalGitProjectAdapter { pub fn open_or_init(project_root: PathBuf) -> Result<Self, LibraryError> }
```

## Visibility

- `pub`

## Docstring

Open the git repo at `project_root`, or `git init` if no
`.git/` directory exists yet. Returns the adapter.

The initial commit is *not* created here — `commit_path` is
the first commit on a freshly-initialised repo. The migration
flow in the app layer is responsible for the "Initial commit
(Oxide import)" call after `git init` when the user first
opts into version control.

## Source
Lines 67–88 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
