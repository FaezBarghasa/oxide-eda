---
okf_version: "0.2"
type: Function
title: open
description: "A file could not be opened — unreadable, unparseable, or empty"
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/open
language: rust
---

# open

A file could not be opened — unreadable, unparseable, or empty

## Signature

```rust
impl ErrorNotice { pub fn open(detail: impl Into<String>) -> Self }
```

## Visibility

- `pub`

## Docstring

A file could not be opened — unreadable, unparseable, or empty
where it must not be (#532).

## Source
Lines 176–181 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
