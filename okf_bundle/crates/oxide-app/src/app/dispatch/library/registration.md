---
okf_version: "0.2"
type: Module
title: registration
description: Library lifecycle handlers — creating a library for a project or
resource: crates/oxide-app/src/app/dispatch/library/registration.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/registration
language: rust
---

# registration

Library lifecycle handlers — creating a library for a project or

## Docstring

Library lifecycle handlers — creating a library for a project or
at a path, the open-library picker, adding standalone symbol /
footprint files, and registering a standalone library onto the
active project.

Extracted verbatim from the library dispatcher (`dispatch/library`);
pure code motion, zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [handle_prompt_library_create_options](/crates/oxide-app/src/app/dispatch/library/registration/handle_prompt_library_create_options.md) |
| related | [handle_library_create_options_toggle_lfs](/crates/oxide-app/src/app/dispatch/library/registration/handle_library_create_options_toggle_lfs.md) |
| related | [handle_library_create_options_toggle_git](/crates/oxide-app/src/app/dispatch/library/registration/handle_library_create_options_toggle_git.md) |
| related | [handle_library_create_options_confirm](/crates/oxide-app/src/app/dispatch/library/registration/handle_library_create_options_confirm.md) |
| related | [handle_create_library_for_project](/crates/oxide-app/src/app/dispatch/library/registration/handle_create_library_for_project.md) |
| related | [handle_create_library_at_path](/crates/oxide-app/src/app/dispatch/library/registration/handle_create_library_at_path.md) |
| related | [handle_picker_message](/crates/oxide-app/src/app/dispatch/library/registration/handle_picker_message.md) |
| related | [handle_add_library_symbol_file_picked](/crates/oxide-app/src/app/dispatch/library/registration/handle_add_library_symbol_file_picked.md) |
| related | [handle_add_library_footprint_file_picked](/crates/oxide-app/src/app/dispatch/library/registration/handle_add_library_footprint_file_picked.md) |
| related | [register_standalone_library_on_project](/crates/oxide-app/src/app/dispatch/library/registration/register_standalone_library_on_project.md) |
| related | [handle_prompt_library_create_options](/crates/oxide-app/src/app/dispatch/library/registration/handle_prompt_library_create_options.md) |
| related | [handle_library_create_options_toggle_lfs](/crates/oxide-app/src/app/dispatch/library/registration/handle_library_create_options_toggle_lfs.md) |
| related | [handle_library_create_options_toggle_git](/crates/oxide-app/src/app/dispatch/library/registration/handle_library_create_options_toggle_git.md) |
| related | [handle_library_create_options_confirm](/crates/oxide-app/src/app/dispatch/library/registration/handle_library_create_options_confirm.md) |
| related | [handle_create_library_for_project](/crates/oxide-app/src/app/dispatch/library/registration/handle_create_library_for_project.md) |
| related | [handle_create_library_at_path](/crates/oxide-app/src/app/dispatch/library/registration/handle_create_library_at_path.md) |
| related | [handle_picker_message](/crates/oxide-app/src/app/dispatch/library/registration/handle_picker_message.md) |
| related | [handle_add_library_symbol_file_picked](/crates/oxide-app/src/app/dispatch/library/registration/handle_add_library_symbol_file_picked.md) |
| related | [handle_add_library_footprint_file_picked](/crates/oxide-app/src/app/dispatch/library/registration/handle_add_library_footprint_file_picked.md) |
| related | [register_standalone_library_on_project](/crates/oxide-app/src/app/dispatch/library/registration/register_standalone_library_on_project.md) |
