---
okf_version: "0.2"
type: Function
title: write_valid_snxsym
description: "Write a valid single-symbol `.snxsym` envelope to `path`."
resource: crates/oxide-app/tests/regression/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:39Z"
concept_id: crates/oxide-app/tests/regression/project/write_valid_snxsym
language: rust
---

# write_valid_snxsym

Write a valid single-symbol `.snxsym` envelope to `path`.

## Signature

```rust
fn write_valid_snxsym(path: &Path, name: &str)
```

## Docstring

Write a valid single-symbol `.snxsym` envelope to `path`.

## Source
Lines 876–881 in `crates/oxide-app/tests/regression/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/tests/regression/project.md) |
| called_by | [a_valid_snxsym_leaves_the_error_card_clear](/crates/oxide-app/tests/regression/project/a_valid_snxsym_leaves_the_error_card_clear.md) |
| called_by | [opening_snxsym_still_creates_editable_tab](/crates/oxide-app/tests/regression/project/opening_snxsym_still_creates_editable_tab.md) |
