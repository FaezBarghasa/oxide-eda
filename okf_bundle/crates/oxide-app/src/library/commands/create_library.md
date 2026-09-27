---
okf_version: "0.2"
type: Function
title: create_library
description: Convenience wrapper — create a project-local library named
resource: crates/oxide-app/src/library/commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/commands/create_library
language: rust
---

# create_library

Convenience wrapper — create a project-local library named

## Signature

```rust
pub fn create_library(
    state: &mut LibraryState,
    project: &mut ProjectData,
    name: &str,
) -> Result<Uuid, LibraryError>
```

## Visibility

- `pub`

## Docstring

Convenience wrapper — create a project-local library named
`<name>` under `<project.dir>/<name>.snxlib`. Keeps the legacy
call sites that don't go through the Save-As dialog working
(currently none — all new code goes through `create_library_at`,
which lets the user pick the location).

## Source
Lines 402–422 in `crates/oxide-app/src/library/commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [commands](/crates/oxide-app/src/library/commands.md) |
| calls | [create_library_at](/crates/oxide-app/src/library/commands/create_library_at.md) |
