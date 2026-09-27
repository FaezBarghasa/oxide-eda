---
okf_version: "0.2"
type: Function
title: a_standing_failure_does_not_flood_the_messages_panel
description: "[test]"
resource: crates/oxide-app/src/app/dir_listing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dir_listing/a_standing_failure_does_not_flood_the_messages_panel
language: rust
---

# a_standing_failure_does_not_flood_the_messages_panel

[test]

## Signature

```rust
fn a_standing_failure_does_not_flood_the_messages_panel()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 148–168 in `crates/oxide-app/src/app/dir_listing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dir_listing](/crates/oxide-app/src/app/dir_listing.md) |
| calls | [unique_dir](/crates/oxide-app/src/app/dir_listing/unique_dir.md) |
| calls | [init_logging](/crates/oxide-app/src/diagnostics/init_logging.md) |
| calls | [list_dir_or_report](/crates/oxide-app/src/app/dir_listing/list_dir_or_report.md) |
