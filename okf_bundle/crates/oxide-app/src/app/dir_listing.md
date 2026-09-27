---
okf_version: "0.2"
type: Module
title: dir_listing
description: "Directory listing that tells \"not there\" apart from \"could not be read\"."
resource: crates/oxide-app/src/app/dir_listing.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dir_listing
language: rust
---

# dir_listing

Directory listing that tells "not there" apart from "could not be read".

## Docstring

Directory listing that tells "not there" apart from "could not be read".

`std::fs::read_dir(dir).ok()` collapses a permission error, a broken
mount and a stale network path into the same empty listing an absent
directory produces. Every caller of that shape in the app feeds a
picker or a tree, so the failure renders as "there is nothing here"
— the user is told their standard libraries, their symbols or their
sibling footprints do not exist while they are sitting on disk.

`NotFound` really does mean empty (it is the normal first-run state)
and stays silent. Everything else reaches the Messages panel.

## Relationships

| Type | Target |
|------|--------|
| related | [list_dir_or_report](/crates/oxide-app/src/app/dir_listing/list_dir_or_report.md) |
| related | [claim_first_report](/crates/oxide-app/src/app/dir_listing/claim_first_report.md) |
| related | [unique_dir](/crates/oxide-app/src/app/dir_listing/unique_dir.md) |
| related | [records_mentioning](/crates/oxide-app/src/app/dir_listing/records_mentioning.md) |
| related | [a_missing_directory_lists_empty_and_says_nothing](/crates/oxide-app/src/app/dir_listing/a_missing_directory_lists_empty_and_says_nothing.md) |
| related | [an_unreadable_directory_reaches_the_messages_panel](/crates/oxide-app/src/app/dir_listing/an_unreadable_directory_reaches_the_messages_panel.md) |
| related | [a_standing_failure_does_not_flood_the_messages_panel](/crates/oxide-app/src/app/dir_listing/a_standing_failure_does_not_flood_the_messages_panel.md) |
