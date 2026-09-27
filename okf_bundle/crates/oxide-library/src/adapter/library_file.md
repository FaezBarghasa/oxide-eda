---
okf_version: "0.2"
type: Function
title: library_file
description: "Borrow the parsed `.snxlib` view if this adapter is backed by an"
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/library_file
language: rust
---

# library_file

Borrow the parsed `.snxlib` view if this adapter is backed by an

## Signature

```rust
fn library_file(&self) -> Option<&LibraryFile>
```

## Docstring

Borrow the parsed `.snxlib` view if this adapter is backed by an
on-disk file. Returns `None` for non-file adapters (e.g. the DB
backend) — they don't have a `[tables.<name>]` document on disk.

Stage 2 introduces this hook so future adapters and callers can
reach the new format without going through the legacy
`manifest()` synthesis. The default `None` keeps existing
adapters compiling unchanged.

## Source
Lines 163–165 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
