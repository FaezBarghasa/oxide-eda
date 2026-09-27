---
okf_version: "0.2"
type: Function
title: emit_bookmarks
description: "Emit `/Outlines` and every outline item into `pdf`. `root_id`"
resource: crates/oxide-output/src/pdf/bookmarks/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/bookmarks/mod/emit_bookmarks
language: rust
---

# emit_bookmarks

Emit `/Outlines` and every outline item into `pdf`. `root_id`

## Signature

```rust
pub(crate) fn emit_bookmarks(
    pdf: &mut Pdf,
    bookmarks: &[PendingBookmark],
    root_id: Ref,
    bookmark_id_base: i32,
    page_refs: &[Ref],
    opts: &PdfOptions,
)
```

## Visibility

- `pub(crate)`

## Docstring

Emit `/Outlines` and every outline item into `pdf`. `root_id`
must already be referenced from the catalog dict via
`catalog.outlines(root_id)` before calling this. `bookmark_id_base`
is the Ref number assigned to `bookmarks[0]`; subsequent items
land at `bookmark_id_base + i`.

## Source
Lines 279–366 in `crates/oxide-output/src/pdf/bookmarks/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod.md) |
| calls | [bookmark_zoom_factor](/crates/oxide-output/src/pdf/bookmarks/mod/bookmark_zoom_factor.md) |
| called_by | [export](/crates/oxide-output/src/pdf/mod/export.md) |
