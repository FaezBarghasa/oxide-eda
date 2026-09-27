---
okf_version: "0.2"
type: Function
title: cancel
description: Mark the flow as cancelled. The owning thread mutates the
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/cancel
language: rust
---

# cancel

Mark the flow as cancelled. The owning thread mutates the

## Signature

```rust
impl CancelHandle { pub fn cancel(&self) }
```

## Visibility

- `pub`

## Docstring

Mark the flow as cancelled. The owning thread mutates the
shared flag directly via `Arc<AtomicBool>` in production; this
helper is kept as a convenient handle for tests + future
in-process cancel paths.

## Source
Lines 89–91 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
