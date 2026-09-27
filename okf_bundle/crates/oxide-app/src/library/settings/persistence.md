---
okf_version: "0.2"
type: Module
title: persistence
description: User-config persistence for the Distributor APIs panel.
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence
language: rust
---

# persistence

User-config persistence for the Distributor APIs panel.

## Docstring

User-config persistence for the Distributor APIs panel.

Stores the `[distributor_apis] preferred_order = [...]` list at
`<config_dir>/oxide/distributors.toml`.

Why a dedicated file vs. piggy-backing on `prefs.json`:
- `prefs.json` is JSON; the rest of the v0.9 library config is TOML
(matching `Manifest`).
- The file gets cross-tool-readable lifecycle defaults in v0.9.1
(AVL, template defaults, etc.); keeping it TOML now lets the
schema grow without a migration.

Errors are intentionally swallowed at the boundary because:
- On startup, a missing/corrupt config must not block the app —
we fall back to [`DistributorSettings::default()`].
- On save, an I/O failure is non-critical — the next save will
just overwrite. A `tracing::warn!` surfaces the why.

## Relationships

| Type | Target |
|------|--------|
| related | [DistributorsConfig](/crates/oxide-app/src/library/settings/persistence/DistributorsConfig.md) |
| related | [DistributorApisSection](/crates/oxide-app/src/library/settings/persistence/DistributorApisSection.md) |
| related | [default](/crates/oxide-app/src/library/settings/persistence/default.md) |
| related | [default](/crates/oxide-app/src/library/settings/persistence/default.md) |
| related | [default_order_strs](/crates/oxide-app/src/library/settings/persistence/default_order_strs.md) |
| related | [default_order](/crates/oxide-app/src/library/settings/persistence/default_order.md) |
| related | [source_to_str](/crates/oxide-app/src/library/settings/persistence/source_to_str.md) |
| related | [str_to_source](/crates/oxide-app/src/library/settings/persistence/str_to_source.md) |
| related | [config_path](/crates/oxide-app/src/library/settings/persistence/config_path.md) |
| related | [config_path_for_dir](/crates/oxide-app/src/library/settings/persistence/config_path_for_dir.md) |
| related | [load_preferred_order](/crates/oxide-app/src/library/settings/persistence/load_preferred_order.md) |
| related | [load_preferred_order_at](/crates/oxide-app/src/library/settings/persistence/load_preferred_order_at.md) |
| related | [save_preferred_order](/crates/oxide-app/src/library/settings/persistence/save_preferred_order.md) |
| related | [save_preferred_order_at](/crates/oxide-app/src/library/settings/persistence/save_preferred_order_at.md) |
| related | [round_trip_writes_and_reads_back](/crates/oxide-app/src/library/settings/persistence/round_trip_writes_and_reads_back.md) |
| related | [save_preferred_order_at_leaves_original_intact_when_write_fails](/crates/oxide-app/src/library/settings/persistence/save_preferred_order_at_leaves_original_intact_when_write_fails.md) |
| related | [missing_file_returns_default_order](/crates/oxide-app/src/library/settings/persistence/missing_file_returns_default_order.md) |
| related | [corrupt_file_returns_default_order](/crates/oxide-app/src/library/settings/persistence/corrupt_file_returns_default_order.md) |
| related | [unknown_distributor_strings_filtered_out](/crates/oxide-app/src/library/settings/persistence/unknown_distributor_strings_filtered_out.md) |
| related | [empty_after_filter_falls_back_to_default](/crates/oxide-app/src/library/settings/persistence/empty_after_filter_falls_back_to_default.md) |
| related | [duplicates_are_collapsed](/crates/oxide-app/src/library/settings/persistence/duplicates_are_collapsed.md) |
| related | [source_str_round_trip_covers_every_variant](/crates/oxide-app/src/library/settings/persistence/source_str_round_trip_covers_every_variant.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
