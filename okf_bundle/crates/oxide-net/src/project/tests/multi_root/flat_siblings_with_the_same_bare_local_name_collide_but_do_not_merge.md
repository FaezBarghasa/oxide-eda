---
okf_version: "0.2"
type: Function
title: flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge
description: "4 ── Two flat pages that both happen to carry the SAME bare local `Net`"
resource: crates/oxide-net/src/project/tests/multi_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/multi_root/flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge
language: rust
---

# flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge

4 ── Two flat pages that both happen to carry the SAME bare local `Net`

## Signature

```rust
fn flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge()
```

## Decorators

- `test`

## Docstring

4 ── Two flat pages that both happen to carry the SAME bare local `Net`
name must NOT merge (a local label is sheet-scoped) — they collide on
the *name* instead, exactly like two same-named nets on one sheet, and
dedup suffixes the second.
[test]

## Source
Lines 176–212 in `crates/oxide-net/src/project/tests/multi_root.rs`

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
| calls | [stitch_pages](/crates/oxide-net/src/project/tests/stitch_pages.md) |
| calls | [names](/crates/oxide-net/src/project/tests/names.md) |
