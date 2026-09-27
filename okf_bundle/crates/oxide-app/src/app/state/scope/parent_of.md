---
okf_version: "0.2"
type: Function
title: parent_of
description: "`child path → parent path` over every loaded sheet, built from the parents'"
resource: crates/oxide-app/src/app/state/scope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:25Z"
concept_id: crates/oxide-app/src/app/state/scope/parent_of
language: rust
---

# parent_of

`child path → parent path` over every loaded sheet, built from the parents'

## Signature

```rust
fn parent_of(loaded: &HashMap<PathBuf, Vec<String>>) -> HashMap<String, PathBuf>
```

## Docstring

`child path → parent path` over every loaded sheet, built from the parents'
own `child_sheets` references (each resolved against the parent's
directory, matching `project_sheets::project_graph`).

`loaded` is a `HashMap`, whose iteration order is per-instance random —
sorted by parent path first so that when two parents claim the same
resolved child key, the first-wins tie-break (lexicographically-smallest
parent path) is the same every run, matching the identical hazard fixed in
`project_sheets::project_graph`.

## Source
Lines 70–84 in `crates/oxide-app/src/app/state/scope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scope](/crates/oxide-app/src/app/state/scope.md) |
| calls | [path_key](/crates/oxide-app/src/app/state/scope/path_key.md) |
| called_by | [project_owning_sheet](/crates/oxide-app/src/app/state/scope/project_owning_sheet.md) |
