---
okf_version: "0.2"
type: Function
title: poll_ready
resource: crates/oxide-library-server/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:36:00Z"
concept_id: crates/oxide-library-server/src/lib/poll_ready
language: rust
---

# poll_ready

## Signature

```rust
impl BearerAuthMiddleware<S> { fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> }
```

## Type Parameters

- `S`
- `B`

## Source
Lines 128–133 in `crates/oxide-library-server/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-library-server/src/lib.md) |
