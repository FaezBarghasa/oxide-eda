---
okf_version: "0.2"
type: Function
title: write_project
description: "Serialize `data` to `path` as pretty JSON. Companion of"
resource: crates/oxide-types/src/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:17Z"
concept_id: crates/oxide-types/src/project/write_project
language: rust
---

# write_project

Serialize `data` to `path` as pretty JSON. Companion of

## Signature

```rust
pub fn write_project(path: &Path, data: &ProjectData) -> Result<(), ProjectError>
```

## Visibility

- `pub`

## Docstring

Serialize `data` to `path` as pretty JSON. Companion of
[`parse_project`] for the JSON-backed branch — newly-added sheets,
PCB, and libraries persist through this writer.

Written atomically (temp file + fsync + rename): the `.snxprj` is
the workspace's index of sheets/PCB/libraries/variants, so a crash
mid-write must never truncate it to a partial file — that would
silently drop the project's contents on the next open.

## Source
Lines 566–575 in `crates/oxide-types/src/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-types/src/project.md) |
| calls | [atomic_write](/crates/oxide-types/src/atomic_io/atomic_write.md) |
| called_by | [write_project_round_trips_atomically_leaving_no_tmp](/crates/oxide-types/src/project/write_project_round_trips_atomically_leaving_no_tmp.md) |
