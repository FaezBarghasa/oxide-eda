---
okf_version: "0.2"
type: Function
title: missing_label
description: "F24 — surface a `(missing)` suffix on every leaf whose backing"
resource: crates/oxide-app/src/panels/projects.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/projects/missing_label
language: rust
---

# missing_label

F24 — surface a `(missing)` suffix on every leaf whose backing

## Signature

```rust
fn missing_label(filename: &str, is_missing: bool) -> String
```

## Docstring

F24 — surface a `(missing)` suffix on every leaf whose backing
file is registered on the project but absent from disk. Catches
orphan references (e.g. user moved/deleted a file outside Oxide,
or a previous library-create attempt left an entry behind without
the file). User sees the broken state at a glance instead of
having to double-click and read an error.

## Source
Lines 154–160 in `crates/oxide-app/src/panels/projects.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [projects](/crates/oxide-app/src/panels/projects.md) |
| called_by | [project_root_node](/crates/oxide-app/src/panels/projects/project_root_node.md) |
