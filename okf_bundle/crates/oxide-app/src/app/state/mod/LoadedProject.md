---
okf_version: "0.2"
type: Class
title: LoadedProject
description: "One loaded project in the multi-project workspace. `path` is the"
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/LoadedProject
language: rust
---

# LoadedProject

One loaded project in the multi-project workspace. `path` is the

## Signature

```rust
pub struct LoadedProject
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

One loaded project in the multi-project workspace. `path` is the
canonical identity (`.standard_pro` / `.snxprj` location on disk); `data`
is the parsed project contents. Multiple projects with different
`path`s coexist in `DocumentState.projects`; two identical `path`s
at once is a loader bug (existing `open_project_file` de-dupes).
[derive(Debug, Clone)]

## Methods

- `id`
- `path`
- `data`
- `pending_libraries`

## Source
Lines 256–270 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
