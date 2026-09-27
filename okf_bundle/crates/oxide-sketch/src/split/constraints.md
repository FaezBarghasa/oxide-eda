---
okf_version: "0.2"
type: Module
title: constraints
description: "Constraint carry-over for a completed [`super::SplitCtx`] split —"
resource: crates/oxide-sketch/src/split/constraints.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/constraints
language: rust
---

# constraints

Constraint carry-over for a completed [`super::SplitCtx`] split —

## Docstring

Constraint carry-over for a completed [`super::SplitCtx`] split —
rewrites, duplicates, or drops every constraint that named the
retired Line so none dangles. See [`super::split_line`]'s doc
comment for the per-kind table this module implements.

## Relationships

| Type | Target |
|------|--------|
| related | [split_constraints](/crates/oxide-sketch/src/split/constraints/split_constraints.md) |
| related | [is_dropped_midpoint](/crates/oxide-sketch/src/split/constraints/is_dropped_midpoint.md) |
| related | [split_constraint](/crates/oxide-sketch/src/split/constraints/split_constraint.md) |
| related | [split_duplicated](/crates/oxide-sketch/src/split/constraints/split_duplicated.md) |
| related | [duplicate](/crates/oxide-sketch/src/split/constraints/duplicate.md) |
| related | [split_point_on_line](/crates/oxide-sketch/src/split/constraints/split_point_on_line.md) |
| related | [point_param](/crates/oxide-sketch/src/split/constraints/point_param.md) |
| related | [retarget_relational](/crates/oxide-sketch/src/split/constraints/retarget_relational.md) |
