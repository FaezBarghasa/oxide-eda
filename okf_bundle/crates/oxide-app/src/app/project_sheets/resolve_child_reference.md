---
okf_version: "0.2"
type: Function
title: resolve_child_reference
description: "Resolve a `ChildSheet.filename` reference against the directory of the sheet"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/resolve_child_reference
language: rust
---

# resolve_child_reference

Resolve a `ChildSheet.filename` reference against the directory of the sheet

## Signature

```rust
pub(crate) fn resolve_child_reference(parent_dir: &Path, child_filename: &str) -> Option<PathBuf>
```

## Visibility

- `pub(crate)`

## Docstring

Resolve a `ChildSheet.filename` reference against the directory of the sheet
that carries it — the single definition of child-reference resolution shared
by the project graph assembly ([`project_graph`]) and in-app
Open Child Sheet navigation (`resolve_child_sheet_path`). Sharing one
definition is what keeps navigation landing on the same file the netlist
stitched for a given reference, instead of the two agreeing only by
coincidence.

- An empty reference (after trimming) yields `None`.
- Both an absolute reference and a relative one are joined onto
`parent_dir` (`Path::join` already replaces `parent_dir` outright when the
reference is absolute, so one join covers both shapes) and the result is
lexically normalized — `.`/`..` resolved without touching the
filesystem, since the child file may not exist yet and
`std::fs::canonicalize` both requires existence and adds a `\\?\` prefix
on Windows.
- A reference whose normalized path escapes `parent_dir` — a `..`
traversal, or an absolute path elsewhere entirely — is rejected: logged
through the app's normal error-surfacing path and returned as `None`,
the same as a reference that does not resolve to a loaded sheet.
- An empty `parent_dir` (an unsaved tab with no project loaded, or a bare
filename tab — `resolve_child_sheet_path`'s `unwrap_or_default()`) has
no real directory to contain anything against, so it is rejected
outright: `Path::starts_with` treats an empty path as a prefix of *any*
path (zero components to match), so checking containment against it is
vacuously true and would let an absolute or `..`-escaping reference
through unchecked (#463, re-opened at this exact boundary).
- Lexical normalization resolves `.`/`..` components only; it does not
touch the filesystem, so a symlink planted inside `parent_dir` that
points back out is not detected here. That residual is accepted: it
requires an attacker who can already write inside the project
directory.

## Source
Lines 475–515 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| calls | [lexically_normalize](/crates/oxide-app/src/app/project_sheets/lexically_normalize.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
| called_by | [resolve_child_sheet_path](/crates/oxide-app/src/app/handlers/canvas/mod/resolve_child_sheet_path.md) |
| called_by | [navigation_and_project_graph_agree_on_the_same_reference](/crates/oxide-app/src/app/project_sheets/navigation_and_project_graph_agree_on_the_same_reference.md) |
| called_by | [project_graph](/crates/oxide-app/src/app/project_sheets/project_graph.md) |
