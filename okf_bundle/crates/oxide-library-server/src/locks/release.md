---
okf_version: "0.2"
type: Function
title: release
resource: crates/oxide-library-server/src/locks.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library-server/src/locks/release
language: rust
---

# release

## Signature

```rust
impl LockManager { pub fn release(&self, uuid: Uuid, field_set: FieldSet, holder: &str) -> Result<(), LockError> }
```

## Visibility

- `pub`

## Source
Lines 148–165 in `crates/oxide-library-server/src/locks.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [locks](/crates/oxide-library-server/src/locks.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
