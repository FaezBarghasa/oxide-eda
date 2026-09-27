---
okf_version: "0.2"
type: Function
title: open_schematic_file
resource: crates/oxide-app/src/app/handlers/document_files/open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/open/open_schematic_file
language: rust
---

# open_schematic_file

## Signature

```rust
impl Oxide { fn open_schematic_file(&mut self, path: PathBuf) -> Result<iced::Task<Message>> }
```

## Source
Lines 275–348 in `crates/oxide-app/src/app/handlers/document_files/open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open](/crates/oxide-app/src/app/handlers/document_files/open.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
| calls | [read_and_parse_schematic](/crates/oxide-app/src/app/handlers/document_files/open/read_and_parse_schematic.md) |
