---
okf_version: "0.2"
type: Class
title: DependencyError
description: "Errors arising during dependency resolution, fetching, locking, or mounting."
resource: crates/oxide-library/src/dependency/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:41Z"
concept_id: crates/oxide-library/src/dependency/mod/DependencyError
language: rust
---

# DependencyError

Errors arising during dependency resolution, fetching, locking, or mounting.

## Signature

```rust
pub enum DependencyError
```

## Decorators

- `derive(Debug, Error)`

## Visibility

- `pub`

## Docstring

Errors arising during dependency resolution, fetching, locking, or mounting.
[derive(Debug, Error)]

## Methods

- `path`
- `source`
- `name`
- `reason`
- `name`
- `expected`
- `actual`
- `name`

## Source
Lines 20–49 in `crates/oxide-library/src/dependency/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dependency](/crates/oxide-library/src/dependency/mod.md) |
