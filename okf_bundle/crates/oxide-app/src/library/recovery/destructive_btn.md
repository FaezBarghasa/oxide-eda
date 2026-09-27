---
okf_version: "0.2"
type: Function
title: destructive_btn
description: "Destructive button — used for *Re-init (lose history)*. Painted"
resource: crates/oxide-app/src/library/recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/recovery/destructive_btn
language: rust
---

# destructive_btn

Destructive button — used for *Re-init (lose history)*. Painted

## Signature

```rust
fn destructive_btn(label: &'a str, message: LibraryMessage) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

Destructive button — used for *Re-init (lose history)*. Painted
muted-red so the lossy semantics read at a glance. Confirm with
the user before tweaking the colour.

## Source
Lines 436–452 in `crates/oxide-app/src/library/recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [recovery](/crates/oxide-app/src/library/recovery.md) |
