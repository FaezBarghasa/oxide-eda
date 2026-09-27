---
okf_version: "0.2"
type: Function
title: rect_visible
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/rect_visible
language: rust
---

# rect_visible

## Signature

```rust
fn rect_visible(min: iced::Point, size: iced::Size, bounds: Rectangle) -> bool
```

## Source
Lines 715–720 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| called_by | [build_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/snapshot/build_renderer_snapshot.md) |
