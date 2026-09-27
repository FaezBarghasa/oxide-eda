---
okf_version: "0.2"
type: Function
title: apply_rounded_corners
description: "Non-Windows stub for [`apply_rounded_corners`]. macOS already rounds"
resource: crates/oxide-app/src/chrome.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/chrome/apply_rounded_corners_1
language: rust
---

# apply_rounded_corners

Non-Windows stub for [`apply_rounded_corners`]. macOS already rounds

## Signature

```rust
pub fn apply_rounded_corners(_id: Id) -> Task<M>
```

## Type Parameters

- `M: 'static + Send`

## Decorators

- `cfg(not(windows))`

## Visibility

- `pub`

## Docstring

Non-Windows stub for [`apply_rounded_corners`]. macOS already rounds
top-level windows itself and Linux rounding is WM-dependent, so there
is no attribute to set and the window id goes unread — hence `_id`.
Replace with a real implementation if the transparent-window +
rounded-container fallback described in the module docs ever lands.
[cfg(not(windows))]

## Source
Lines 35–37 in `crates/oxide-app/src/chrome.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chrome](/crates/oxide-app/src/chrome.md) |
