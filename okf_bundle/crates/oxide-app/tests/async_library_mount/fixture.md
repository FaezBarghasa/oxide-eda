---
okf_version: "0.2"
type: Function
title: fixture
description: "A real `.snxlib` written by the real Oxide writers. Returns the"
resource: crates/oxide-app/tests/async_library_mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/async_library_mount/fixture
language: rust
---

# fixture

A real `.snxlib` written by the real Oxide writers. Returns the

## Signature

```rust
fn fixture(tag: &'static str) -> (tempfile::TempDir, PathBuf)
```

## Docstring

A real `.snxlib` written by the real Oxide writers. Returns the
tempdir too — dropping it deletes the library out from under the test.

## Source
Lines 35–46 in `crates/oxide-app/tests/async_library_mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [async_library_mount](/crates/oxide-app/tests/async_library_mount.md) |
| calls | [generate_library](/crates/oxide-app/tests/support/mod/generate_library.md) |
| called_by | [a_double_click_during_a_silent_mount_upgrades_the_intent](/crates/oxide-app/tests/async_library_mount/a_double_click_during_a_silent_mount_upgrades_the_intent.md) |
| called_by | [a_silent_request_never_downgrades_an_open_browser_tab_intent](/crates/oxide-app/tests/async_library_mount/a_silent_request_never_downgrades_an_open_browser_tab_intent.md) |
| called_by | [close_library_cancels_a_mount_still_being_prepared](/crates/oxide-app/tests/async_library_mount/close_library_cancels_a_mount_still_being_prepared.md) |
| called_by | [mount_prepared_does_not_duplicate_an_already_mounted_library](/crates/oxide-app/tests/async_library_mount/mount_prepared_does_not_duplicate_an_already_mounted_library.md) |
| called_by | [prepare_mount_then_mount_prepared_matches_open_library](/crates/oxide-app/tests/async_library_mount/prepare_mount_then_mount_prepared_matches_open_library.md) |
| called_by | [request_mount_asks_for_a_spawn_once_then_reports_in_flight](/crates/oxide-app/tests/async_library_mount/request_mount_asks_for_a_spawn_once_then_reports_in_flight.md) |
| called_by | [request_mount_reports_already_mounted_and_records_nothing](/crates/oxide-app/tests/async_library_mount/request_mount_reports_already_mounted_and_records_nothing.md) |
| called_by | [take_mount_intent_is_one_shot](/crates/oxide-app/tests/async_library_mount/take_mount_intent_is_one_shot.md) |
