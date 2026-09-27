---
okf_version: "0.2"
type: Class
title: TabPillStyle
description: Visual state that drives bg + border colour. Built as a struct so
resource: crates/oxide-widgets/src/tab_pill.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/tab_pill/TabPillStyle
language: rust
---

# TabPillStyle

Visual state that drives bg + border colour. Built as a struct so

## Signature

```rust
pub struct TabPillStyle
```

## Decorators

- `derive(Debug, Clone, Copy)`

## Visibility

- `pub`

## Docstring

Visual state that drives bg + border colour. Built as a struct so
callers don't have to care about the exact derivation rules
(e.g. drag-tinted active = mix-of-accent-and-fill).
[derive(Debug, Clone, Copy)]

## Methods

- `fill`
- `border`
- `accent`
- `is_active`
- `is_last`
- `accent_position`

## Source
Lines 48–60 in `crates/oxide-widgets/src/tab_pill.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tab_pill](/crates/oxide-widgets/src/tab_pill.md) |
