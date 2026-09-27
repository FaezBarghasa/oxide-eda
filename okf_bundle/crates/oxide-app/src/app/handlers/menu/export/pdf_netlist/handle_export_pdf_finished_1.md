---
okf_version: "0.2"
type: Function
title: handle_export_pdf_finished
resource: crates/oxide-app/src/app/handlers/menu/export/pdf_netlist.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/pdf_netlist/handle_export_pdf_finished_1
language: rust
---

# handle_export_pdf_finished

## Signature

```rust
pub(crate) fn handle_export_pdf_finished(
        &mut self,
        result: Result<PathBuf, String>,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 24–115 in `crates/oxide-app/src/app/handlers/menu/export/pdf_netlist.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdf_netlist](/crates/oxide-app/src/app/handlers/menu/export/pdf_netlist.md) |
| calls | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
| calls | [log_stitch_issues](/crates/oxide-app/src/app/handlers/menu/export/mod/log_stitch_issues.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
