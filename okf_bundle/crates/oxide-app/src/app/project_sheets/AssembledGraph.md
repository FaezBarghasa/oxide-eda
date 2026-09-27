---
okf_version: "0.2"
type: Class
title: AssembledGraph
description: "The result of [`project_graph`] re-keying [`ProjectSheetSet::sheets`]"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/AssembledGraph
language: rust
---

# AssembledGraph

The result of [`project_graph`] re-keying [`ProjectSheetSet::sheets`]

## Signature

```rust
pub(crate) struct AssembledGraph
```

## Visibility

- `pub(crate)`

## Docstring

The result of [`project_graph`] re-keying [`ProjectSheetSet::sheets`]
(`PathBuf -> SchematicSheet`) into [`oxide_net::ProjectGraph`]'s `sheets`
/ `resolved` inputs — plus what went wrong while re-keying, and the
reverse map back to a path for navigation / `unreadable` messages.

## Methods

- `sheets`
- `resolved`
- `issues`

## Source
Lines 240–249 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
