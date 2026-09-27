---
okf_version: "0.2"
type: Function
title: unique_dir
description: Unique directory name so the shared diagnostics ring can be
resource: crates/oxide-app/src/app/dir_listing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dir_listing/unique_dir
language: rust
---

# unique_dir

Unique directory name so the shared diagnostics ring can be

## Signature

```rust
fn unique_dir(tag: &str) -> PathBuf
```

## Docstring

Unique directory name so the shared diagnostics ring can be
asserted on while other tests write to it. Kept short: the panel
compacts a record to 160 characters and the path has to survive.

## Source
Lines 90–93 in `crates/oxide-app/src/app/dir_listing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dir_listing](/crates/oxide-app/src/app/dir_listing.md) |
| called_by | [a_missing_directory_lists_empty_and_says_nothing](/crates/oxide-app/src/app/dir_listing/a_missing_directory_lists_empty_and_says_nothing.md) |
| called_by | [a_standing_failure_does_not_flood_the_messages_panel](/crates/oxide-app/src/app/dir_listing/a_standing_failure_does_not_flood_the_messages_panel.md) |
| called_by | [an_unreadable_directory_reaches_the_messages_panel](/crates/oxide-app/src/app/dir_listing/an_unreadable_directory_reaches_the_messages_panel.md) |
