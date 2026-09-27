---
okf_version: "0.2"
type: Class
title: LibGraphic
description: "A graphic primitive inside a library symbol, tagged with unit and body-style"
resource: crates/oxide-types/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:42Z"
concept_id: crates/oxide-types/src/schematic/mod/LibGraphic
language: rust
---

# LibGraphic

A graphic primitive inside a library symbol, tagged with unit and body-style

## Signature

```rust
pub struct LibGraphic
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A graphic primitive inside a library symbol, tagged with unit and body-style
so the renderer can filter to only draw the correct unit for each instance.

- `unit == 0`       → common to ALL units (always rendered)
- `unit == N`       → only rendered for symbol instances with `unit = N`
- `body_style == 0` → common to all body styles (normal + De Morgan)
- `body_style == 1` → normal body style (default)
- `body_style == 2` → De Morgan body style
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `unit`
- `body_style`
- `graphic`

## Source
Lines 463–469 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
