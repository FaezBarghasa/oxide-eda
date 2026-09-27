---
okf_version: "0.2"
type: Function
title: set_entity_role
description: "Mutates the entity's role attrs in place. Pure — no solver work."
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch/set_entity_role
language: rust
---

# set_entity_role

Mutates the entity's role attrs in place. Pure — no solver work.

## Signature

```rust
pub fn set_entity_role(footprint: &mut Footprint, id: SketchEntityId, role: RoleTag)
```

## Visibility

- `pub`

## Docstring

Mutates the entity's role attrs in place. Pure — no solver work.
Visible to tests so they can assert the shape of the resulting
Entity without spinning up a solve.

## Source
Lines 102–252 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [apply_sketch_role](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_role.md) |
