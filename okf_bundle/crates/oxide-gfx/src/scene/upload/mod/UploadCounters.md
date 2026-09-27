---
okf_version: "0.2"
type: Class
title: UploadCounters
description: Per-category upload/update counters for instrumentation and tests.
resource: crates/oxide-gfx/src/scene/upload/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T19:56:21Z"
concept_id: crates/oxide-gfx/src/scene/upload/mod/UploadCounters
language: rust
---

# UploadCounters

Per-category upload/update counters for instrumentation and tests.

## Signature

```rust
pub struct UploadCounters
```

## Decorators

- `derive(Clone, Copy, Debug, Default, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Per-category upload/update counters for instrumentation and tests.
[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]

## Methods

- `line_uploads`
- `circle_uploads`
- `arc_uploads`
- `polygon_uploads`
- `text_uploads`
- `grid_refreshes`
- `overlay_line_uploads`
- `overlay_circle_uploads`
- `overlay_polygon_uploads`
- `erc_marker_line_uploads`
- `erc_marker_circle_uploads`
- `erc_marker_polygon_uploads`
- `theme_refreshes`

## Source
Lines 207–221 in `crates/oxide-gfx/src/scene/upload/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [upload](/crates/oxide-gfx/src/scene/upload/mod.md) |
