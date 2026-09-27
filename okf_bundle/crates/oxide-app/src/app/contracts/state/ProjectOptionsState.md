---
okf_version: "0.2"
type: Class
title: ProjectOptionsState
description: "State for the read-only \"Project Options\" modal — the v0.9 surface"
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/ProjectOptionsState
language: rust
---

# ProjectOptionsState

State for the read-only "Project Options" modal — the v0.9 surface

## Signature

```rust
pub struct ProjectOptionsState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

State for the read-only "Project Options" modal — the v0.9 surface
is a metadata summary (name / dir / schematic root / pcb file /
libraries). Editing happens through the dedicated rename / library
flows; a future revision can promote this to a full editor.
[derive(Debug, Clone)]

## Methods

- `project_idx`
- `name`
- `directory`
- `schematic_root`
- `pcb_file`
- `library_count`

## Source
Lines 363–370 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
