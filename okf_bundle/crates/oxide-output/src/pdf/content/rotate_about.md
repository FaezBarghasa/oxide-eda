---
okf_version: "0.2"
type: Function
title: rotate_about
resource: crates/oxide-output/src/pdf/content.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/pdf/content/rotate_about
language: rust
---

# rotate_about

## Signature

```rust
fn rotate_about(px: f32, py: f32, ox: f32, oy: f32, rotation_deg: f32) -> (f32, f32)
```

## Source
Lines 350–357 in `crates/oxide-output/src/pdf/content.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [content](/crates/oxide-output/src/pdf/content.md) |
| called_by | [build_page_content](/crates/oxide-output/src/pdf/content/build_page_content.md) |
