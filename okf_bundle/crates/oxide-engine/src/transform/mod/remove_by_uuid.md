---
okf_version: "0.2"
type: Function
title: remove_by_uuid
resource: crates/oxide-engine/src/transform/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/transform/mod/remove_by_uuid
language: rust
---

# remove_by_uuid

## Signature

```rust
fn remove_by_uuid(items: &mut Vec<T>, uuid: uuid::Uuid) -> bool
```

## Type Parameters

- `T`

## Source
Lines 527–534 in `crates/oxide-engine/src/transform/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-engine/src/transform/mod.md) |
| called_by | [remove_selected_item](/crates/oxide-engine/src/transform/mod/remove_selected_item.md) |
