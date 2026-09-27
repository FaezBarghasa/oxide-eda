---
okf_version: "0.2"
type: Class
title: BumpKind
description: Severity classification for a drift entry — drives the default
resource: crates/oxide-app/src/library/updates_dialog.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/updates_dialog/BumpKind
language: rust
---

# BumpKind

Severity classification for a drift entry — drives the default

## Signature

```rust
pub enum BumpKind
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Severity classification for a drift entry — drives the default
checkbox state and the warning glyph in the row label. Pure-data
(no semver crate dependency); see [`classify_bump`] for the
inference rule.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 63–74 in `crates/oxide-app/src/library/updates_dialog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates_dialog](/crates/oxide-app/src/library/updates_dialog.md) |
