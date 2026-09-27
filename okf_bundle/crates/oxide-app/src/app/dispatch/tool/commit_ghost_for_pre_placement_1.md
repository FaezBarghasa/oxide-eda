---
okf_version: "0.2"
type: Function
title: commit_ghost_for_pre_placement
resource: crates/oxide-app/src/app/dispatch/tool.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/tool/commit_ghost_for_pre_placement_1
language: rust
---

# commit_ghost_for_pre_placement

## Signature

```rust
fn commit_ghost_for_pre_placement(
        &mut self,
        kind: crate::panels::PrePlacementKind,
        label_text: &str,
    )
```

## Decorators

- `expect(
        dead_code,
        reason = "kept until the selection-aware Properties panel renders full per-kind fields"
    )`

## Source
Lines 15–155 in `crates/oxide-app/src/app/dispatch/tool.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tool](/crates/oxide-app/src/app/dispatch/tool.md) |
