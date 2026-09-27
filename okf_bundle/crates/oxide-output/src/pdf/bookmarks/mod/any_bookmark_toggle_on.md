---
okf_version: "0.2"
type: Function
title: any_bookmark_toggle_on
description: Returns true when at least one toggle that produces a bookmark
resource: crates/oxide-output/src/pdf/bookmarks/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/bookmarks/mod/any_bookmark_toggle_on
language: rust
---

# any_bookmark_toggle_on

Returns true when at least one toggle that produces a bookmark

## Signature

```rust
fn any_bookmark_toggle_on(opts: &PdfOptions) -> bool
```

## Docstring

Returns true when at least one toggle that produces a bookmark
is enabled.

## Source
Lines 370–372 in `crates/oxide-output/src/pdf/bookmarks/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod.md) |
| called_by | [build_bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod/build_bookmarks.md) |
