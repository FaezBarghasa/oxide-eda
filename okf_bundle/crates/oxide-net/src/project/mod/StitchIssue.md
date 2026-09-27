---
okf_version: "0.2"
type: Class
title: StitchIssue
description: A structural problem found while stitching. The netlist is still produced
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/StitchIssue
language: rust
---

# StitchIssue

A structural problem found while stitching. The netlist is still produced

## Signature

```rust
pub enum StitchIssue
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

A structural problem found while stitching. The netlist is still produced
(best-effort, deterministic); issues tell consumers where it is degraded.
[derive(Debug, Clone, PartialEq, Eq)]

## Methods

- `parent_path`
- `sheet_name`
- `filename`
- `parent_path`
- `filename`
- `filename_a`
- `filename_b`
- `filename`
- `reference`
- `name`
- `key`
- `kept`
- `dropped`

## Source
Lines 136–182 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
