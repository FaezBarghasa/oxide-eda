---
okf_version: "0.2"
type: Module
title: split
description: "`split_line` — divide a sketch `Line` at a parameter into two Lines"
resource: crates/oxide-sketch/src/split/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/mod
language: rust
---

# split

`split_line` — divide a sketch `Line` at a parameter into two Lines

## Docstring

`split_line` — divide a sketch `Line` at a parameter into two Lines
sharing a new mid `Point`.

Pure model primitive (issue #360) — no `oxide-app` / UI dependency.
The footprint editor's Break Track action (issue #372) is the
consumer: it hit-tests a click against a Line, projects it to a
parameter `t`, and calls [`split_line`] directly.

## Relationships

| Type | Target |
|------|--------|
| related | [SplitError](/crates/oxide-sketch/src/split/mod/SplitError.md) |
| related | [SplitResult](/crates/oxide-sketch/src/split/mod/SplitResult.md) |
| related | [split_line](/crates/oxide-sketch/src/split/mod/split_line.md) |
| related | [commit_split](/crates/oxide-sketch/src/split/mod/commit_split.md) |
| related | [build_split_entities](/crates/oxide-sketch/src/split/mod/build_split_entities.md) |
| related | [ValidatedSplit](/crates/oxide-sketch/src/split/mod/ValidatedSplit.md) |
| related | [validate_split](/crates/oxide-sketch/src/split/mod/validate_split.md) |
| related | [mm_distance](/crates/oxide-sketch/src/split/mod/mm_distance.md) |
| related | [entity_point_xy](/crates/oxide-sketch/src/split/mod/entity_point_xy.md) |
| related | [SplitCtx](/crates/oxide-sketch/src/split/mod/SplitCtx.md) |
