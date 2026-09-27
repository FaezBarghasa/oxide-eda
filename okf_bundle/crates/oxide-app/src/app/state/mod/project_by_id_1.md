---
okf_version: "0.2"
type: Function
title: project_by_id
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/project_by_id_1
language: rust
---

# project_by_id

## Signature

```rust
pub fn project_by_id(&self, id: ProjectId) -> Option<&LoadedProject>
```

## Visibility

- `pub`

## Source
Lines 641–643 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
