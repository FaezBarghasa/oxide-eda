---
okf_version: "0.2"
type: Class
title: TrackItem
description: One file/directory entry surfaced on the Enable Version Control
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/TrackItem
language: rust
---

# TrackItem

One file/directory entry surfaced on the Enable Version Control

## Signature

```rust
pub struct TrackItem
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

One file/directory entry surfaced on the Enable Version Control
picker. The user can opt items out of the initial commit by
untoggling `tracked`; untracked entries get written into a
generated `.gitignore` so they sit outside the repo from day one.
[derive(Debug, Clone)]

## Methods

- `absolute`
- `relative`
- `label`
- `is_directory`
- `tracked`

## Source
Lines 293–303 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
