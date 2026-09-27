---
okf_version: "0.2"
type: Function
title: snapshot
resource: crates/oxide-library-server/src/locks.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library-server/src/locks/snapshot_1
language: rust
---

# snapshot

## Signature

```rust
pub fn snapshot(&self, uuid: Uuid, field_set: FieldSet) -> Option<LockSnapshot>
```

## Visibility

- `pub`

## Source
Lines 167–181 in `crates/oxide-library-server/src/locks.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [locks](/crates/oxide-library-server/src/locks.md) |
