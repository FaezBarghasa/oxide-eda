---
okf_version: "0.2"
type: Function
title: resolve_page_range_preview
resource: crates/oxide-output/src/preview/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/preview/mod/resolve_page_range_preview
language: rust
---

# resolve_page_range_preview

## Signature

```rust
fn resolve_page_range_preview(range: &PageRange, sheet_count: usize) -> Vec<usize>
```

## Source
Lines 70–97 in `crates/oxide-output/src/preview/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-output/src/preview/mod.md) |
| called_by | [rasterize](/crates/oxide-output/src/preview/mod/rasterize.md) |
