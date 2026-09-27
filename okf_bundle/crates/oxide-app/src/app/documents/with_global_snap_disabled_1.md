---
okf_version: "0.2"
type: Function
title: with_global_snap_disabled
description: "HI-23: builder that seeds `global_snap_disabled` from"
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/with_global_snap_disabled_1
language: rust
---

# with_global_snap_disabled

HI-23: builder that seeds `global_snap_disabled` from

## Signature

```rust
pub fn with_global_snap_disabled(mut self, disabled: bool) -> Self
```

## Visibility

- `pub`

## Docstring

HI-23: builder that seeds `global_snap_disabled` from
`ui_state.snap_enabled` so opening a `.snxfpt` while the user
has the global snap toggle off doesn't surprise them with snap
suddenly back on. Call sites pass `!ui_state.snap_enabled`.

## Source
Lines 572–575 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
