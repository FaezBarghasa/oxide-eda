---
okf_version: "0.2"
type: Function
title: atomic_write
description: "Atomically write `bytes` to `path`. Creates parent directories"
resource: crates/oxide-types/src/atomic_io.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/atomic_io/atomic_write
language: rust
---

# atomic_write

Atomically write `bytes` to `path`. Creates parent directories

## Signature

```rust
pub fn atomic_write(path: &Path, bytes: &[u8]) -> io::Result<()>
```

## Visibility

- `pub`

## Docstring

Atomically write `bytes` to `path`. Creates parent directories
(if any) as a side effect — matches `std::fs::write` ergonomics.

## Source
Lines 65–110 in `crates/oxide-types/src/atomic_io.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [atomic_io](/crates/oxide-types/src/atomic_io.md) |
| calls | [tmp_path_for](/crates/oxide-types/src/atomic_io/tmp_path_for.md) |
| called_by | [a_post_creation_failure_leaves_no_stray_tmp](/crates/oxide-types/src/atomic_io/a_post_creation_failure_leaves_no_stray_tmp.md) |
| called_by | [creates_parent_directory](/crates/oxide-types/src/atomic_io/creates_parent_directory.md) |
| called_by | [overwrites_existing_file](/crates/oxide-types/src/atomic_io/overwrites_existing_file.md) |
| called_by | [round_trip_creates_destination](/crates/oxide-types/src/atomic_io/round_trip_creates_destination.md) |
| called_by | [writes_empty_and_large_payloads_and_leaves_no_tmp](/crates/oxide-types/src/atomic_io/writes_empty_and_large_payloads_and_leaves_no_tmp.md) |
| called_by | [write_project](/crates/oxide-types/src/project/write_project.md) |
