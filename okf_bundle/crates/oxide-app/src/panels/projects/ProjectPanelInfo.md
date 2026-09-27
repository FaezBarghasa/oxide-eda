---
okf_version: "0.2"
type: Class
title: ProjectPanelInfo
description: Per-project bundle surfaced to the Projects panel. One entry per
resource: crates/oxide-app/src/panels/projects.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/projects/ProjectPanelInfo
language: rust
---

# ProjectPanelInfo

Per-project bundle surfaced to the Projects panel. One entry per

## Signature

```rust
pub struct ProjectPanelInfo
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Per-project bundle surfaced to the Projects panel. One entry per
`LoadedProject` in `DocumentState.projects`. `build_project_tree`
iterates this list to emit one tree root per project.
[derive(Debug, Clone)]

## Methods

- `id`
- `name`
- `project_file`
- `project_file_open`
- `project_file_dirty`
- `project_file_active`
- `project_file_missing`
- `pcb_file`
- `pcb_file_open`
- `pcb_file_dirty`
- `pcb_file_active`
- `pcb_file_missing`
- `sheets`
- `libraries`
- `is_active`
- `is_dirty`

## Source
Lines 37–74 in `crates/oxide-app/src/panels/projects.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [projects](/crates/oxide-app/src/panels/projects.md) |
