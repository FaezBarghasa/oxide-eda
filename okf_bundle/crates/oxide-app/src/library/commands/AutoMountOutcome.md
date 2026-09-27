---
okf_version: "0.2"
type: Class
title: AutoMountOutcome
description: "What one `auto_mount_project_libraries` pass decided — issue #99"
resource: crates/oxide-app/src/library/commands.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/commands/AutoMountOutcome
language: rust
---

# AutoMountOutcome

What one `auto_mount_project_libraries` pass decided — issue #99

## Signature

```rust
pub struct AutoMountOutcome
```

## Decorators

- `derive(Debug, Default, Clone)`

## Visibility

- `pub`

## Docstring

What one `auto_mount_project_libraries` pass decided — issue #99
part 2c.

Two numbers instead of one count, because a cold mount is no longer
finished when this function returns: it has only been *recorded*. A
single `mounted: usize` would have claimed work that has not happened
yet. Total libraries handled is `refreshed + pending.len()`.
[derive(Debug, Default, Clone)]

## Methods

- `refreshed`
- `pending`

## Source
Lines 432–439 in `crates/oxide-app/src/library/commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [commands](/crates/oxide-app/src/library/commands.md) |
