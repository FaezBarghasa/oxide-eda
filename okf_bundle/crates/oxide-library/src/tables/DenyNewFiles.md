---
okf_version: "0.2"
type: Class
title: DenyNewFiles
description: "RAII guard: while alive, `dir` rejects new-file creation, so"
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/DenyNewFiles
language: rust
---

# DenyNewFiles

RAII guard: while alive, `dir` rejects new-file creation, so

## Signature

```rust
struct DenyNewFiles
```

## Docstring

RAII guard: while alive, `dir` rejects new-file creation, so
`atomic_write`'s `File::create(&tmp)` fails deterministically
regardless of the unique per-writer temp name it picks (#416).
Restores permissions on drop.

On Windows, `icacls`'s exit code is not proof the deny is enforced
yet — under full-workspace parallel test load its write to the
directory's security descriptor can lose the race against the very
next `File::create` in the same directory (#482), so `on` settles
the deny with a real probe write before returning.

## Methods

- `dir`

## Source
Lines 491–493 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
