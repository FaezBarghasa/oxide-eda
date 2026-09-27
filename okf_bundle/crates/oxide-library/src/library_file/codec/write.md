---
okf_version: "0.2"
type: Function
title: write
description: Serialize back to a TOML document. Output is deterministic — calling
resource: crates/oxide-library/src/library_file/codec.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/codec/write
language: rust
---

# write

Serialize back to a TOML document. Output is deterministic — calling

## Signature

```rust
impl LibraryFile { pub fn write(&self) -> Result<String, LibraryFileError> }
```

## Visibility

- `pub`

## Docstring

Serialize back to a TOML document. Output is deterministic — calling
`parse` on the result yields a `LibraryFile` equal to `self`.

The header is emitted via `toml::to_string_pretty`; each table is
appended as a `[tables.<name>]` section with the TSV wrapped in a
TOML literal multi-line string (`'''…'''`) so newlines and
backslashes survive without escaping noise. Tables are emitted in
`BTreeMap` order — sorted by name.

## Source
Lines 82–151 in `crates/oxide-library/src/library_file/codec.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [codec](/crates/oxide-library/src/library_file/codec.md) |
| calls | [truncate](/crates/oxide-output/examples/qa_harness/truncate.md) |
| calls | [serialize_tsv](/crates/oxide-library/src/library_file/codec/serialize_tsv.md) |
