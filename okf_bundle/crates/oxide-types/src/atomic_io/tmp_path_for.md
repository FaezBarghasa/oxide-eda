---
okf_version: "0.2"
type: Function
title: tmp_path_for
description: "Build a temp sibling path for `path`, unique per call: `<pid>-<counter>`"
resource: crates/oxide-types/src/atomic_io.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/atomic_io/tmp_path_for
language: rust
---

# tmp_path_for

Build a temp sibling path for `path`, unique per call: `<pid>-<counter>`

## Signature

```rust
fn tmp_path_for(path: &Path) -> io::Result<PathBuf>
```

## Docstring

Build a temp sibling path for `path`, unique per call: `<pid>-<counter>`
is appended before `.tmp` so concurrent writers to the same target
never collide on the same temp name, while the rename below stays a
same-directory (same-filesystem) atomic-replace.

## Source
Lines 50–61 in `crates/oxide-types/src/atomic_io.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [atomic_io](/crates/oxide-types/src/atomic_io.md) |
| called_by | [atomic_write](/crates/oxide-types/src/atomic_io/atomic_write.md) |
| called_by | [tmp_path_for_is_unique_per_call](/crates/oxide-types/src/atomic_io/tmp_path_for_is_unique_per_call.md) |
