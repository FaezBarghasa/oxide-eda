---
okf_version: "0.2"
type: Function
title: primitive_name_in_envelope
description: "Name of the first primitive inside a `.snxfpt` / `.snxsim` envelope."
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/primitive_name_in_envelope
language: rust
---

# primitive_name_in_envelope

Name of the first primitive inside a `.snxfpt` / `.snxsim` envelope.

## Signature

```rust
fn primitive_name_in_envelope(
    kind: PrimitiveKind,
    bytes: &[u8],
    file_name: &str,
) -> Result<String, LibraryError>
```

## Docstring

Name of the first primitive inside a `.snxfpt` / `.snxsim` envelope.

`file_name` is diagnostic only — it names the offending file in the
error text, which is what the user sees when a container is corrupt.
A well-formed envelope carrying *zero* primitives is corrupt, not
empty, and must fail loudly rather than vanish from the listing.

HI-8: [`PrimitiveKind::Symbol`] cannot arrive here. Symbol
containers are `<slug>.snxsym`, so the uuid-stem parse in
`list_primitive_summaries` would skip them anyway, and
`list_symbols` walks them through `scan_symbol_files` instead. The
match is spelled out per variant with no `_` arm, so a kind added
to [`PrimitiveKind`] later is a compile error here rather than a
listing that silently comes back empty.

## Source
Lines 493–530 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
| called_by | [list_primitive_summaries](/crates/oxide-library/src/adapters/local_git/primitives/list_primitive_summaries.md) |
