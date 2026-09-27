---
okf_version: "0.2"
type: Function
title: adapter_ref
description: "Widen a concrete adapter to the trait object `reload_tables` takes."
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/adapter_ref
language: rust
---

# adapter_ref

Widen a concrete adapter to the trait object `reload_tables` takes.

## Signature

```rust
fn adapter_ref(adapter: &LocalGitAdapter) -> &dyn LibraryAdapter
```

## Docstring

Widen a concrete adapter to the trait object `reload_tables` takes.
Separate fn only so the coercion site reads clearly.

## Source
Lines 176–178 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
| called_by | [prepare_mount](/crates/oxide-app/src/library/mount/prepare_mount.md) |
