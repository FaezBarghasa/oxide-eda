---
okf_version: "0.2"
type: Function
title: list_primitives_skips_non_uuid_and_foreign_files
description: "Files whose stem is not a uuid, and files with a foreign extension,"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/list_primitives_skips_non_uuid_and_foreign_files
language: rust
---

# list_primitives_skips_non_uuid_and_foreign_files

Files whose stem is not a uuid, and files with a foreign extension,

## Signature

```rust
fn list_primitives_skips_non_uuid_and_foreign_files()
```

## Decorators

- `test`

## Docstring

Files whose stem is not a uuid, and files with a foreign extension,
are skipped rather than surfaced or errored on. A `README.md` or a
stray editor backup dropped into `footprints/` must not break the
Library Browser.
[test]

## Source
Lines 468–482 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [init_adapter](/crates/oxide-library/tests/local_git_adapter/init_adapter.md) |
| calls | [fixture_footprint](/crates/oxide-library/tests/local_git_adapter/fixture_footprint.md) |
