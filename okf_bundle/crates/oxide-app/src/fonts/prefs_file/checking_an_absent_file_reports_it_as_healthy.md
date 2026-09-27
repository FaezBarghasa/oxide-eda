---
okf_version: "0.2"
type: Function
title: checking_an_absent_file_reports_it_as_healthy
description: "Absence is the fresh-install path the writer starts from, so the"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/checking_an_absent_file_reports_it_as_healthy
language: rust
---

# checking_an_absent_file_reports_it_as_healthy

Absence is the fresh-install path the writer starts from, so the

## Signature

```rust
fn checking_an_absent_file_reports_it_as_healthy()
```

## Decorators

- `test`

## Docstring

Absence is the fresh-install path the writer starts from, so the
probe must not raise a banner over it.
[test]

## Source
Lines 572–585 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [check_prefs_file_at](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file_at.md) |
