---
okf_version: "0.2"
type: Function
title: hash_row_content
description: Compute the canonical content hash of a row.
resource: crates/oxide-library/src/hash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/hash/hash_row_content
language: rust
---

# hash_row_content

Compute the canonical content hash of a row.

## Signature

```rust
pub fn hash_row_content(row: &ComponentRow) -> Result<[u8; 32], LibraryError>
```

## Visibility

- `pub`

## Docstring

Compute the canonical content hash of a row.

Returns `LibraryError::Backend` when the row contains a non-finite float
(`NaN` / `±Infinity`) inside any `ParamValue::Number` / `ParamValue::
Measurement` reached by the canonical view. Two reasons we trap this here
rather than upstream:

1. **Hash determinism.** `serde_json` silently encodes `NaN` / `Infinity`
as JSON `null`, so a row carrying `Number(NaN)` would hash equal to a
row carrying `Number(0.0)` (after the upstream constructor zeroed it
out, etc.) — that defeats the "did the technical content actually
change?" question content_hash answers.
2. **Less-invasive than upstream validation.** Tightening `ParamValue` to
reject non-finite floats at construction would require making variants
`#[non_exhaustive]` and rewriting ~50 enum-literal call sites in
oxide-app and tests. Boundary validation here keeps the change
localised to the two functions whose semantics actually depend on it.

## Source
Lines 91–109 in `crates/oxide-library/src/hash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hash](/crates/oxide-library/src/hash.md) |
| calls | [check_param_map_finite](/crates/oxide-library/src/hash/check_param_map_finite.md) |
| called_by | [handle_browser_cell_commit](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_cell_commit.md) |
| called_by | [handle_browser_edit_msg](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_edit_msg.md) |
| called_by | [apply_primitive_pick_to_browser_row](/crates/oxide-app/src/app/dispatch/library/primitive_picker/apply_primitive_pick_to_browser_row.md) |
| called_by | [apply_primitive_pick_to_preview](/crates/oxide-app/src/app/dispatch/library/primitive_picker/apply_primitive_pick_to_preview.md) |
| called_by | [create_component_row](/crates/oxide-app/src/library/commands/create_component_row.md) |
| called_by | [refresh_content_hash](/crates/oxide-library/src/component/refresh_content_hash.md) |
| called_by | [hash_returns_backend_error_on_non_finite_float](/crates/oxide-library/src/hash/hash_returns_backend_error_on_non_finite_float.md) |
| called_by | [import_to_library](/crates/oxide-library/src/scraper/import_to_library.md) |
