# hash

## Classs

- [CanonView](CanonView.md) — Canonical serialisation view — only the fields the hash should care about.

## Functions

- [check_param_map_finite](check_param_map_finite.md) — Reject any `ParamValue::Number` / `ParamValue::Measurement` whose float
- [fixture_row](fixture_row.md)
- [from_row](from_row.md)
- [from_row](from_row_1.md)
- [hash_changes_when_class_changes](hash_changes_when_class_changes.md) — [test]
- [hash_changes_when_pin_map_changes](hash_changes_when_pin_map_changes.md) — [test]
- [hash_changes_when_primary_mpn_changes](hash_changes_when_primary_mpn_changes.md) — [test]
- [hash_changes_when_state_changes](hash_changes_when_state_changes.md) — [test]
- [hash_changes_when_symbol_ref_changes](hash_changes_when_symbol_ref_changes.md) — [test]
- [hash_ignores_timestamps_and_self_hash](hash_ignores_timestamps_and_self_hash.md) — Bookkeeping fields (`created`, `updated`, `content_hash`) MUST NOT
- [hash_is_deterministic](hash_is_deterministic.md) — [test]
- [hash_returns_backend_error_on_non_finite_float](hash_returns_backend_error_on_non_finite_float.md) — `serde_json` cannot encode `NaN` / `±Infinity`. The hash function
- [hash_row_content](hash_row_content.md) — Compute the canonical content hash of a row.
