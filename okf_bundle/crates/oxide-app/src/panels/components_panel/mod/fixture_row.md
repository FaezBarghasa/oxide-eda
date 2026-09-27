---
okf_version: "0.2"
type: Function
title: fixture_row
resource: crates/oxide-app/src/panels/components_panel/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/mod/fixture_row
language: rust
---

# fixture_row

## Signature

```rust
fn fixture_row(pn: &str, mpn: &str, mfr: &str) -> ComponentRow
```

## Source
Lines 387–413 in `crates/oxide-app/src/panels/components_panel/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/panels/components_panel/mod.md) |
| called_by | [row_matches_empty_needle_passes_everything](/crates/oxide-app/src/panels/components_panel/mod/row_matches_empty_needle_passes_everything.md) |
| called_by | [row_matches_internal_pn](/crates/oxide-app/src/panels/components_panel/mod/row_matches_internal_pn.md) |
| called_by | [row_matches_library_name](/crates/oxide-app/src/panels/components_panel/mod/row_matches_library_name.md) |
| called_by | [row_matches_manufacturer_case_insensitive](/crates/oxide-app/src/panels/components_panel/mod/row_matches_manufacturer_case_insensitive.md) |
| called_by | [row_matches_mpn_substring](/crates/oxide-app/src/panels/components_panel/mod/row_matches_mpn_substring.md) |
| called_by | [row_matches_no_hit_returns_false](/crates/oxide-app/src/panels/components_panel/mod/row_matches_no_hit_returns_false.md) |
