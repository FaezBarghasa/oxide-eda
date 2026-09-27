---
okf_version: "0.2"
type: Function
title: upgrade
description: "Intents only ever escalate. `OpenBrowserTab` absorbs `Silent`;"
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/upgrade_1
language: rust
---

# upgrade

Intents only ever escalate. `OpenBrowserTab` absorbs `Silent`;

## Signature

```rust
pub fn upgrade(self, other: Self) -> Self
```

## Visibility

- `pub`

## Docstring

Intents only ever escalate. `OpenBrowserTab` absorbs `Silent`;
`Silent` never downgrades an `OpenBrowserTab` already recorded.

## Source
Lines 60–65 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
