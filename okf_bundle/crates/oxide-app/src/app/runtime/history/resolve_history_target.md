---
okf_version: "0.2"
type: Function
title: resolve_history_target
description: "Resolve the active tab into a `(project_dir, rel_path)` pair the"
resource: crates/oxide-app/src/app/runtime/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/history/resolve_history_target
language: rust
---

# resolve_history_target

Resolve the active tab into a `(project_dir, rel_path)` pair the

## Signature

```rust
fn resolve_history_target(app: &super::super::Oxide) -> Option<HistoryTarget>
```

## Docstring

Resolve the active tab into a `(project_dir, rel_path)` pair the
History panel can hand to `oxide_library::project_file_history`.

Discovery walks parent directories looking for a `.git/`. We stop
at the first ancestor that has one — that's the git working tree
the file participates in. For library-rooted files (`.snxsym` /
`.snxfpt` etc.) the `.git/` typically sits at the `.snxlib`
directory; for project files it sits at the project root.

Returns `None` for tab kinds that don't correspond to an
on-disk file we want to track (e.g. ComponentEditor — the
row-shaped editor doesn't write an addressable file in v1).

## Source
Lines 145–190 in `crates/oxide-app/src/app/runtime/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/app/runtime/history.md) |
| called_by | [refresh_history_panel](/crates/oxide-app/src/app/runtime/history/refresh_history_panel.md) |
