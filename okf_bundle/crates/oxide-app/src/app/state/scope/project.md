---
okf_version: "0.2"
type: Function
title: project
resource: crates/oxide-app/src/app/state/scope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:25Z"
concept_id: crates/oxide-app/src/app/state/scope/project
language: rust
---

# project

## Signature

```rust
fn project(id: u32, dir: &str, filenames: &[&str]) -> LoadedProject
```

## Source
Lines 119–146 in `crates/oxide-app/src/app/state/scope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scope](/crates/oxide-app/src/app/state/scope.md) |
| calls | [ProjectId](/crates/oxide-app/src/app/state/mod/ProjectId.md) |
| called_by | [draw](/crates/oxide-app/src/library/editor/footprint/preview3d/draw.md) |
| called_by | [draw](/crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview/draw.md) |
| called_by | [pad_stack_preview](/crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview/pad_stack_preview.md) |
