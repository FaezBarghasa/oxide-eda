---
okf_version: "0.2"
type: Function
title: parse_project
description: "Parse a `.snxprj` project file. Two paths:"
resource: crates/oxide-types/src/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:17Z"
concept_id: crates/oxide-types/src/project/parse_project
language: rust
---

# parse_project

Parse a `.snxprj` project file. Two paths:

## Signature

```rust
pub fn parse_project(path: &Path) -> Result<ProjectData, ProjectError>
```

## Visibility

- `pub`

## Docstring

Parse a `.snxprj` project file. Two paths:

1. **JSON content** — newer projects ship a serialized [`ProjectData`]
inside the file. We deserialize it directly and patch `dir` to
match the file's actual location (so projects keep working when
copied to a different folder).
2. **Empty / legacy marker** — older `.snxprj` files were empty
markers; the parser inferred everything from the directory
(probe for `<name>.snxsch` / `<name>.snxpcb`). We keep that
fallback so existing projects keep loading.

Standard project files (`.standard_pro`) are not supported in Oxide
Community. Users running Standard projects use the optional
`oxide-standard-import` GPL-3.0 companion tool to convert their files
first.

## Source
Lines 470–556 in `crates/oxide-types/src/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-types/src/project.md) |
| called_by | [load_or_activate_project](/crates/oxide-app/src/app/handlers/document_files/open/load_or_activate_project.md) |
| called_by | [measure_library_open](/crates/oxide-app/tests/measure_library_open/measure_library_open.md) |
| called_by | [f10_save_persists_snxprj_as_valid_json](/crates/oxide-app/tests/regression/project/f10_save_persists_snxprj_as_valid_json.md) |
| called_by | [loaded_project_data_round_trips_via_write_then_parse](/crates/oxide-app/tests/regression/project/loaded_project_data_round_trips_via_write_then_parse.md) |
| called_by | [save_all_writes_dirty_snxprj_and_clears_dirty_marker](/crates/oxide-app/tests/regression/project/save_all_writes_dirty_snxprj_and_clears_dirty_marker.md) |
| called_by | [main](/crates/oxide-output/examples/qa_harness/main.md) |
| called_by | [parse_project_errors_on_corrupt_json_instead_of_degrading](/crates/oxide-types/src/project/parse_project_errors_on_corrupt_json_instead_of_degrading.md) |
| called_by | [parse_project_probes_directory_for_non_json_marker](/crates/oxide-types/src/project/parse_project_probes_directory_for_non_json_marker.md) |
| called_by | [write_project_round_trips_atomically_leaving_no_tmp](/crates/oxide-types/src/project/write_project_round_trips_atomically_leaving_no_tmp.md) |
