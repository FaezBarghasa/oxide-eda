---
okf_version: "0.2"
type: Function
title: root_dir
description: "Absolute path to the directory that *contains* the `.snxlib` file"
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/root_dir
language: rust
---

# root_dir

Absolute path to the directory that *contains* the `.snxlib` file

## Signature

```rust
fn root_dir(&self) -> Option<&std::path::Path>
```

## Docstring

Absolute path to the directory that *contains* the `.snxlib` file
— i.e. the per-library git repo root and parent of `symbols/`,
`footprints/`, `sims/`. Returns `None` for non-file adapters.

Differs from [`Self::root_path`]: under v0.9, `root_path` and
`root_dir` happen to coincide for `LocalGitAdapter` (both point
at the directory holding the `.snxlib` file), but the new name
makes the parent-of-file relationship explicit per
`v0.9-snxlib-as-file-plan.md` §2 Stage B.

## Source
Lines 176–178 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
