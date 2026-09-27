---
okf_version: "0.2"
type: Class
title: LifecycleFilter
description: Lifecycle visibility mode for the Library Browser grid.
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod/LifecycleFilter
language: rust
---

# LifecycleFilter

Lifecycle visibility mode for the Library Browser grid.

## Signature

```rust
pub enum LifecycleFilter
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Eq, Default)`

## Visibility

- `pub`

## Docstring

Lifecycle visibility mode for the Library Browser grid.

Mirrors plan §6 "Lifecycle, tagging, distributors": rows tagged
`Released`/`InReview`/`Draft` count as "active" for filtering, while
`Deprecated` rows are tinted yellow when shown and `Obsolete` rows
are hidden by default. Stage 18 surfaces these as a single dropdown
pill in the browser header so users can pivot the visible row set
without touching every row's lifecycle field.
[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]

## Source
Lines 85–99 in `crates/oxide-app/src/library/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/state/mod.md) |
