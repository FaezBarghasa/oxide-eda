---
okf_version: "0.2"
type: Class
title: VersionControlScope
description: Whether the Enable Version Control modal is initialising a
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/VersionControlScope
language: rust
---

# VersionControlScope

Whether the Enable Version Control modal is initialising a

## Signature

```rust
pub enum VersionControlScope
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Whether the Enable Version Control modal is initialising a
project repo (whole-project tree) or a library repo (a single
`.snxlib` directory). Branches the confirm handler so it can
run `git init` against the right working tree and emit the
scope-appropriate log line.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 311–314 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
