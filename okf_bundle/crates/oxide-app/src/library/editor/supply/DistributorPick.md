---
okf_version: "0.2"
type: Class
title: DistributorPick
description: "Wrapper so the pick_list `Display` impl prints a friendly label"
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/DistributorPick
language: rust
---

# DistributorPick

Wrapper so the pick_list `Display` impl prints a friendly label

## Signature

```rust
struct DistributorPick
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Docstring

Wrapper so the pick_list `Display` impl prints a friendly label
instead of the bare debug variant name.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 75–75 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| called_by | [listing_row](/crates/oxide-app/src/library/editor/supply/listing_row.md) |
