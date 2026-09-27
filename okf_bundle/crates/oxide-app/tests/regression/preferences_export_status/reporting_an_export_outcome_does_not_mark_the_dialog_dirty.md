---
okf_version: "0.2"
type: Function
title: reporting_an_export_outcome_does_not_mark_the_dialog_dirty
description: "The status line is feedback about one export, not an edit. Reporting"
resource: crates/oxide-app/tests/regression/preferences_export_status.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_export_status/reporting_an_export_outcome_does_not_mark_the_dialog_dirty
language: rust
---

# reporting_an_export_outcome_does_not_mark_the_dialog_dirty

The status line is feedback about one export, not an edit. Reporting

## Signature

```rust
fn reporting_an_export_outcome_does_not_mark_the_dialog_dirty()
```

## Decorators

- `test`

## Docstring

The status line is feedback about one export, not an edit. Reporting
it must never trip the unsaved-changes guard, or a failed export
would trap the user in a dialog that refuses to close.
[test]

## Source
Lines 65–82 in `crates/oxide-app/tests/regression/preferences_export_status.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_export_status](/crates/oxide-app/tests/regression/preferences_export_status.md) |
| calls | [inner](/crates/oxide-app/tests/regression/preferences_export_status/inner.md) |
