---
okf_version: "0.2"
type: Function
title: assemble_project_sheets
description: "Every sheet this project consists of: the declared `pages` **union**"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/assemble_project_sheets
language: rust
---

# assemble_project_sheets

Every sheet this project consists of: the declared `pages` **union**

## Signature

```rust
pub(crate) fn assemble_project_sheets(
    document_state: &DocumentState,
    pages: &[PathBuf],
    root_path: &Path,
) -> ProjectSheetSet
```

## Visibility

- `pub(crate)`

## Docstring

Every sheet this project consists of: the declared `pages` **union**
everything reachable from `root_path` and from those pages down the
`child_sheets` graph.

Neither half alone is the project. `pages` (a project's `sheets` list) are
what it *prints*: a hierarchical child is reached by a `child_sheets`
reference and is never added to that list — descending into one opens a tab
without registering it — so the pages alone drop whole subtrees and then
report a `MissingChild` for a sheet sitting in memory or right next to its
parent on disk. Reachability alone is no better: nothing requires a listed
page to be referenced as a child by anything (`project_navigation/add.rs`
appends unconditionally), so a flat project loses every page but the root.

Only paths reached from this project are loaded — an open tab belonging to
some *other* project never rides along. Cycle-safe on the visited set.

## Source
Lines 162–195 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| calls | [walk](/crates/oxide-app/src/app/project_sheets/walk.md) |
| calls | [path_key](/crates/oxide-app/src/app/state/scope/path_key.md) |
| called_by | [refresh_project_netlist](/crates/oxide-app/src/app/mutation_gateway/refresh_project_netlist.md) |
| called_by | [assemble_active_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_active_project_sheets.md) |
