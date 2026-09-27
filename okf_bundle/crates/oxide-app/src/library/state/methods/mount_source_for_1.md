---
okf_version: "0.2"
type: Function
title: mount_source_for
description: Classify how a mounted library got there — drives the
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/mount_source_for_1
language: rust
---

# mount_source_for

Classify how a mounted library got there — drives the

## Signature

```rust
pub fn mount_source_for(
        &self,
        path: &Path,
        project_paths: &[PathBuf],
    ) -> ComponentsMountSource
```

## Visibility

- `pub`

## Docstring

Classify how a mounted library got there — drives the
Components Panel section bucketing (Stage 9). Project
libraries take precedence over Installed/Global so a global
library that's also referenced by the active project
surfaces under the "Project" header.

## Source
Lines 317–335 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
