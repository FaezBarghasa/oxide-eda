---
okf_version: "0.2"
type: Module
title: hash
description: Deterministic content hashing for component rows.
resource: crates/oxide-library/src/hash.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/hash
language: rust
---

# hash

Deterministic content hashing for component rows.

## Docstring

Deterministic content hashing for component rows.

Hash is computed over a canonical JSON serialisation of the row's
binding fields (primitive refs, MPN, supply, parameters, …). Sorted-key
`BTreeMap`s keep output byte-stable across runs.

Excluded from the canon view (intentionally): `created`, `updated`,
`content_hash`. Those are bookkeeping that changes on every save and
would defeat the "did the technical content actually change?" question
the hash answers.

Per `v0.9-refactor-2-plan.md` §6 step 1.6, this replaces the older
`hash_revision_content`. The pattern (canonical JSON over a `Serialize`
view) is identical; only the input type changed from `Revision` to
`ComponentRow`.

## Relationships

| Type | Target |
|------|--------|
| related | [CanonView](/crates/oxide-library/src/hash/CanonView.md) |
| related | [from_row](/crates/oxide-library/src/hash/from_row.md) |
| related | [from_row](/crates/oxide-library/src/hash/from_row.md) |
| related | [hash_row_content](/crates/oxide-library/src/hash/hash_row_content.md) |
| related | [check_param_map_finite](/crates/oxide-library/src/hash/check_param_map_finite.md) |
| related | [fixture_row](/crates/oxide-library/src/hash/fixture_row.md) |
| related | [hash_is_deterministic](/crates/oxide-library/src/hash/hash_is_deterministic.md) |
| related | [hash_changes_when_primary_mpn_changes](/crates/oxide-library/src/hash/hash_changes_when_primary_mpn_changes.md) |
| related | [hash_changes_when_symbol_ref_changes](/crates/oxide-library/src/hash/hash_changes_when_symbol_ref_changes.md) |
| related | [hash_changes_when_pin_map_changes](/crates/oxide-library/src/hash/hash_changes_when_pin_map_changes.md) |
| related | [hash_ignores_timestamps_and_self_hash](/crates/oxide-library/src/hash/hash_ignores_timestamps_and_self_hash.md) |
| related | [hash_changes_when_class_changes](/crates/oxide-library/src/hash/hash_changes_when_class_changes.md) |
| related | [hash_changes_when_state_changes](/crates/oxide-library/src/hash/hash_changes_when_state_changes.md) |
| related | [hash_returns_backend_error_on_non_finite_float](/crates/oxide-library/src/hash/hash_returns_backend_error_on_non_finite_float.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [sha2](/_dependencies/cargo/sha2.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
