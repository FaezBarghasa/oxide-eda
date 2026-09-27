---
okf_version: "0.2"
type: Function
title: autoplace_all_marked_fields
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/autoplace_all_marked_fields
language: rust
---

# autoplace_all_marked_fields

## Signature

```rust
pub(crate) fn autoplace_all_marked_fields(document: &mut oxide_types::schematic::SchematicSheet)
```

## Decorators

- `expect(
    dead_code,
    reason = "waiting on oxide-app to wire the Re-autoplace all fields command"
)`

## Visibility

- `pub(crate)`

## Source
Lines 39–50 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| calls | [autoplace_fields](/crates/oxide-engine/src/transform/autoplace/autoplace_fields.md) |
