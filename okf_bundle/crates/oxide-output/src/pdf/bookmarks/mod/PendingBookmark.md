---
okf_version: "0.2"
type: Class
title: PendingBookmark
description: A bookmark target before any PDF refs have been allocated.
resource: crates/oxide-output/src/pdf/bookmarks/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/bookmarks/mod/PendingBookmark
language: rust
---

# PendingBookmark

A bookmark target before any PDF refs have been allocated.

## Signature

```rust
pub(crate) struct PendingBookmark
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub(crate)`

## Docstring

A bookmark target before any PDF refs have been allocated.
[derive(Debug, Clone)]

## Methods

- `title`
- `parent_idx`
- `children`
- `page_idx`
- `x_pt`
- `y_pt`

## Source
Lines 38–51 in `crates/oxide-output/src/pdf/bookmarks/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod.md) |
