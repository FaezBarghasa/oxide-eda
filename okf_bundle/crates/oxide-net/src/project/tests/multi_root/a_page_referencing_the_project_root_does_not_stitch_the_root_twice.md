---
okf_version: "0.2"
type: Function
title: a_page_referencing_the_project_root_does_not_stitch_the_root_twice
description: "13 ── The same rule applied to the project root: a page that references the"
resource: crates/oxide-net/src/project/tests/multi_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/multi_root/a_page_referencing_the_project_root_does_not_stitch_the_root_twice
language: rust
---

# a_page_referencing_the_project_root_does_not_stitch_the_root_twice

13 ── The same rule applied to the project root: a page that references the

## Signature

```rust
fn a_page_referencing_the_project_root_does_not_stitch_the_root_twice()
```

## Decorators

- `test`

## Docstring

13 ── The same rule applied to the project root: a page that references the
root makes the root that page's child, so the root contributes one
occurrence rather than one as a top-level page plus one nested. The
root is deliberately not pinned to the front of the walk — nothing
downstream needs it to be occurrence 0, and pinning it is exactly what
would duplicate it here.
[test]

## Source
Lines 479–519 in `crates/oxide-net/src/project/tests/multi_root.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_root](/crates/oxide-net/src/project/tests/multi_root.md) |
| calls | [empty_sheet](/crates/oxide-net/src/project/tests/empty_sheet.md) |
| calls | [wire](/crates/oxide-net/src/project/tests/wire.md) |
| calls | [pt](/crates/oxide-net/src/project/tests/pt.md) |
| calls | [label](/crates/oxide-net/src/project/tests/label.md) |
| calls | [add_lib](/crates/oxide-net/src/project/tests/add_lib.md) |
| calls | [place](/crates/oxide-net/src/project/tests/place.md) |
| calls | [child_sheet](/crates/oxide-net/src/project/tests/child_sheet.md) |
| calls | [stitch_pages](/crates/oxide-net/src/project/tests/stitch_pages.md) |
