---
okf_version: "0.2"
type: Function
title: for_adapter
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/for_adapter
language: rust
---

# for_adapter

## Signature

```rust
impl MountKey { fn for_adapter(adapter: &dyn LibraryAdapter) -> Self }
```

## Source
Lines 67–72 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| calls | [library_id](/crates/oxide-library/src/adapter/library_id.md) |
