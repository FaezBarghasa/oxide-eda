---
okf_version: "0.2"
type: Function
title: renew
resource: crates/oxide-library-server/src/locks.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library-server/src/locks/renew_1
language: rust
---

# renew

## Signature

```rust
pub fn renew(&self, uuid: Uuid, field_set: FieldSet, holder: &str) -> Result<(), LockError>
```

## Visibility

- `pub`

## Source
Lines 128–146 in `crates/oxide-library-server/src/locks.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [locks](/crates/oxide-library-server/src/locks.md) |
