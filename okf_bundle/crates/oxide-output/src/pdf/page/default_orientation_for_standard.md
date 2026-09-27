---
okf_version: "0.2"
type: Function
title: default_orientation_for_standard
description: Derive orientation from a Standard paper-size string.
resource: crates/oxide-output/src/pdf/page.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/pdf/page/default_orientation_for_standard
language: rust
---

# default_orientation_for_standard

Derive orientation from a Standard paper-size string.

## Signature

```rust
impl PageSize { pub fn default_orientation_for_standard(s: &str) -> Orientation }
```

## Visibility

- `pub`

## Docstring

Derive orientation from a Standard paper-size string.

Standard schematics default to landscape for A-series except A4 which is
portrait, and landscape for all ANSI sizes. The `portrait` flag in the
Standard `(paper ...)` node overrides this; pass it when present.

## Source
Lines 50–60 in `crates/oxide-output/src/pdf/page.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [page](/crates/oxide-output/src/pdf/page.md) |
