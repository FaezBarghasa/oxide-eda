---
okf_version: "0.2"
type: Function
title: key
description: "A [`oxide_net::SheetKey`] from a literal already in the normalized"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/key
language: rust
---

# key

A [`oxide_net::SheetKey`] from a literal already in the normalized

## Signature

```rust
fn key(k: &str) -> oxide_net::SheetKey
```

## Docstring

A [`oxide_net::SheetKey`] from a literal already in the normalized
form `sheet_key` produces — for asserting against an assembled graph.

## Source
Lines 613–615 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
