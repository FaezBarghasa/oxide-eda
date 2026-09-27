---
okf_version: "0.2"
type: Function
title: apply_rounded_corners
description: Fire-and-forget task that applies OS-native rounded corners to the
resource: crates/oxide-app/src/chrome.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/chrome/apply_rounded_corners
language: rust
---

# apply_rounded_corners

Fire-and-forget task that applies OS-native rounded corners to the

## Signature

```rust
pub fn apply_rounded_corners(id: Id) -> Task<M>
```

## Type Parameters

- `M: 'static + Send`

## Decorators

- `cfg(windows)`

## Visibility

- `pub`

## Docstring

Fire-and-forget task that applies OS-native rounded corners to the
given window id. Returns a `Task<M>` for any `M` so callers can
batch this into whatever message flow they already have.
[cfg(windows)]

## Source
Lines 25–27 in `crates/oxide-app/src/chrome.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chrome](/crates/oxide-app/src/chrome.md) |
| calls | [set_rounded](/crates/oxide-app/src/chrome/set_rounded.md) |
