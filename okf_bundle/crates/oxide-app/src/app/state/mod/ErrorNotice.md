---
okf_version: "0.2"
type: Class
title: ErrorNotice
description: "A user-visible error card: a short heading plus the detail line."
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/ErrorNotice
language: rust
---

# ErrorNotice

A user-visible error card: a short heading plus the detail line.

## Signature

```rust
pub struct ErrorNotice
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

A user-visible error card: a short heading plus the detail line.

One card serves every operation that has to stop and tell the user
something failed. The heading is data rather than a literal in the
view because the card was already shared by operations that are not
exports — see [`DocumentState::error_notice`].
[derive(Debug, Clone, PartialEq, Eq)]

## Methods

- `title`
- `detail`

## Source
Lines 150–155 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
