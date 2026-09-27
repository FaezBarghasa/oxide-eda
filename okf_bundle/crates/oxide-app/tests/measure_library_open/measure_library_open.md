---
okf_version: "0.2"
type: Function
title: measure_library_open
description: "[test]"
resource: crates/oxide-app/tests/measure_library_open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:25Z"
concept_id: crates/oxide-app/tests/measure_library_open/measure_library_open
language: rust
---

# measure_library_open

[test]

## Signature

```rust
fn measure_library_open()
```

## Decorators

- `test`
- `ignore = "timing probe, asserts nothing; run with --release ... -- --ignored --nocapture"`

## Docstring

[test]
[ignore = "timing probe, asserts nothing; run with --release ... -- --ignored --nocapture"]

## Source
Lines 228–394 in `crates/oxide-app/tests/measure_library_open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [measure_library_open](/crates/oxide-app/tests/measure_library_open.md) |
| calls | [timed](/crates/oxide-app/tests/measure_library_open/timed.md) |
| calls | [generate_library](/crates/oxide-app/tests/support/mod/generate_library.md) |
| calls | [primitive_file_sizes](/crates/oxide-app/tests/support/mod/primitive_file_sizes.md) |
| calls | [measure_scale](/crates/oxide-app/tests/measure_library_open/measure_scale.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [write_project](/crates/oxide-app/tests/measure_library_open/write_project.md) |
| calls | [measure](/crates/oxide-app/tests/measure_library_open/measure.md) |
| calls | [parse_project](/crates/oxide-types/src/project/parse_project.md) |
| calls | [auto_mount_project_libraries](/crates/oxide-app/src/library/commands/auto_mount_project_libraries.md) |
| calls | [prepare_mount](/crates/oxide-app/src/library/mount/prepare_mount.md) |
