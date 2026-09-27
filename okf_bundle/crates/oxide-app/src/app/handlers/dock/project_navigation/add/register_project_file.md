---
okf_version: "0.2"
type: Function
title: register_project_file
description: "Push a freshly added file into the project's in-memory model so"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/add/register_project_file
language: rust
---

# register_project_file

Push a freshly added file into the project's in-memory model so

## Signature

```rust
impl Oxide { fn register_project_file(&mut self, project_idx: usize, file_path: &std::path::Path) -> bool }
```

## Docstring

Push a freshly added file into the project's in-memory model so
the tree picks it up. Returns `true` when something was actually
inserted (the caller flips the project dirty bit on `true`).
Files already referenced are skipped — re-adding the same file
is a no-op rather than a duplicate row.

## Source
Lines 298–381 in `crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [add](/crates/oxide-app/src/app/handlers/dock/project_navigation/add.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
