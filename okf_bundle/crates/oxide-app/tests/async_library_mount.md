---
okf_version: "0.2"
type: Module
title: async_library_mount
description: "Issue #99 part 2c — the off-thread `.snxlib` mount."
resource: crates/oxide-app/tests/async_library_mount.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/async_library_mount
language: rust
---

# async_library_mount

Issue #99 part 2c — the off-thread `.snxlib` mount.

## Docstring

Issue #99 part 2c — the off-thread `.snxlib` mount.

Two things need pinning, and they fail in different ways.

**The payload.** Moving the mount off the UI thread only helps if the
library that lands is the same one `open_library` used to build
inline. `prepare_mount_then_mount_prepared_matches_open_library` pins
that against the synchronous path itself rather than against
hand-written counts, so it keeps meaning something if the caches
change shape.

**The bookkeeping.** The rest is about a map that has to survive a
race: a project auto-mount and a user double-click can ask for the
same `.snxlib` at once, and the user can close a library while its
preparation is still running. Those paths have no loud symptom when
they go wrong — a lost tab, a library mounted twice, or a closed
library quietly reappearing — so each one gets a test.

## Relationships

| Type | Target |
|------|--------|
| related | [fixture](/crates/oxide-app/tests/async_library_mount/fixture.md) |
| related | [prepare_mount_then_mount_prepared_matches_open_library](/crates/oxide-app/tests/async_library_mount/prepare_mount_then_mount_prepared_matches_open_library.md) |
| related | [request_mount_asks_for_a_spawn_once_then_reports_in_flight](/crates/oxide-app/tests/async_library_mount/request_mount_asks_for_a_spawn_once_then_reports_in_flight.md) |
| related | [a_double_click_during_a_silent_mount_upgrades_the_intent](/crates/oxide-app/tests/async_library_mount/a_double_click_during_a_silent_mount_upgrades_the_intent.md) |
| related | [a_silent_request_never_downgrades_an_open_browser_tab_intent](/crates/oxide-app/tests/async_library_mount/a_silent_request_never_downgrades_an_open_browser_tab_intent.md) |
| related | [request_mount_reports_already_mounted_and_records_nothing](/crates/oxide-app/tests/async_library_mount/request_mount_reports_already_mounted_and_records_nothing.md) |
| related | [close_library_cancels_a_mount_still_being_prepared](/crates/oxide-app/tests/async_library_mount/close_library_cancels_a_mount_still_being_prepared.md) |
| related | [take_mount_intent_is_one_shot](/crates/oxide-app/tests/async_library_mount/take_mount_intent_is_one_shot.md) |
| related | [mount_prepared_does_not_duplicate_an_already_mounted_library](/crates/oxide-app/tests/async_library_mount/mount_prepared_does_not_duplicate_an_already_mounted_library.md) |
