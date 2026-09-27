---
okf_version: "0.2"
type: Function
title: nets_group_active
description: Returns true when the Nets group has any reason to exist.
resource: crates/oxide-output/src/pdf/bookmarks/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/bookmarks/mod/nets_group_active
language: rust
---

# nets_group_active

Returns true when the Nets group has any reason to exist.

## Signature

```rust
fn nets_group_active(opts: &PdfOptions) -> bool
```

## Docstring

Returns true when the Nets group has any reason to exist.

## Source
Lines 375–378 in `crates/oxide-output/src/pdf/bookmarks/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod.md) |
| called_by | [build_bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod/build_bookmarks.md) |
