---
okf_version: "0.2"
type: Class
title: PreviewTab
description: Component Preview tabs in display order.
resource: crates/oxide-app/src/library/state/preview.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/state/preview/PreviewTab
language: rust
---

# PreviewTab

Component Preview tabs in display order.

## Signature

```rust
pub enum PreviewTab
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Component Preview tabs in display order.

The Component view is preview-only: Symbol and Footprint are
read-only renders; editing happens via the standalone
`.snxsym` / `.snxfpt` document editors.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

## Source
Lines 104–110 in `crates/oxide-app/src/library/state/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/state/preview.md) |
