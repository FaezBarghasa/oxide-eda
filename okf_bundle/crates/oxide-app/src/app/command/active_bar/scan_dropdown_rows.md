---
okf_version: "0.2"
type: Function
title: scan_dropdown_rows
description: "`(visible label, ActiveBarAction variant name)` for every uniform"
resource: crates/oxide-app/src/app/command/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/command/active_bar/scan_dropdown_rows
language: rust
---

# scan_dropdown_rows

`(visible label, ActiveBarAction variant name)` for every uniform

## Signature

```rust
fn scan_dropdown_rows(src: &str) -> Vec<(String, String)>
```

## Docstring

`(visible label, ActiveBarAction variant name)` for every uniform
dropdown row in `src`, from both row-building shapes.

## Source
Lines 397–421 in `crates/oxide-app/src/app/command/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/app/command/active_bar.md) |
| called_by | [catalog_labels_match_the_active_bar_literals](/crates/oxide-app/src/app/command/active_bar/catalog_labels_match_the_active_bar_literals.md) |
