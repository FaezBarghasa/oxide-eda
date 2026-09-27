---
okf_version: "0.2"
type: Function
title: union_bounds
resource: crates/oxide-app/src/app/handlers/clipboard_workflows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/clipboard_workflows/union_bounds
language: rust
---

# union_bounds

## Signature

```rust
fn union_bounds(
    current: Option<oxide_types::schematic::Aabb>,
    next: oxide_types::schematic::Aabb,
) -> Option<oxide_types::schematic::Aabb>
```

## Source
Lines 5–13 in `crates/oxide-app/src/app/handlers/clipboard_workflows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [clipboard_workflows](/crates/oxide-app/src/app/handlers/clipboard_workflows.md) |
| called_by | [clipboard_bounds](/crates/oxide-app/src/app/handlers/clipboard_workflows/clipboard_bounds.md) |
