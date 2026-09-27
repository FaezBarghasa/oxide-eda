---
okf_version: "0.2"
type: Function
title: mutate_library_file
description: "Mutate the in-memory `library_file`, persist it, and commit the"
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/mutate_library_file_1
language: rust
---

# mutate_library_file

Mutate the in-memory `library_file`, persist it, and commit the

## Signature

```rust
pub(super) fn mutate_library_file(
        &self,
        f: F,
        message: &str,
        fallback: &str,
    ) -> Result<(), LibraryError>
```

## Type Parameters

- `F`

## Visibility

- `pub(super)`

## Docstring

Mutate the in-memory `library_file`, persist it, and commit the
`.snxlib` with the supplied message.

## Source
Lines 259–276 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
