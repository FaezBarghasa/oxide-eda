---
okf_version: "0.2"
type: Function
title: handle_export_bom_finished
resource: crates/oxide-app/src/app/handlers/menu/export/bom.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/bom/handle_export_bom_finished
language: rust
---

# handle_export_bom_finished

## Signature

```rust
impl Oxide { pub(crate) fn handle_export_bom_finished(
        &mut self,
        result: Result<PathBuf, String>,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 279–379 in `crates/oxide-app/src/app/handlers/menu/export/bom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-app/src/app/handlers/menu/export/bom.md) |
| calls | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
| calls | [log_stitch_issues](/crates/oxide-app/src/app/handlers/menu/export/mod/log_stitch_issues.md) |
