---
okf_version: "0.2"
type: Class
title: CascadeReport
description: Outcome of one cascade pass — which rows were silently auto-bumped
resource: crates/oxide-library/src/cascade.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/cascade/CascadeReport
language: rust
---

# CascadeReport

Outcome of one cascade pass — which rows were silently auto-bumped

## Signature

```rust
pub struct CascadeReport
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Outcome of one cascade pass — which rows were silently auto-bumped
and which were left alone with a stale `<kind>_version` pin.

The caller (typically `LocalGitAdapter::save_*`) inspects this
after a save to decide whether to surface a UI notice. Stage 15
callers ignore the stale list and let the Library Browser's
existing indicator pick it up; Stage 16 will pump the stale list
into the schematic-side Library Updates dialog.
[derive(Clone, Debug, Default, PartialEq, Eq)]

## Methods

- `auto_bumped`
- `stale`

## Source
Lines 47–55 in `crates/oxide-library/src/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-library/src/cascade.md) |
