# project

## Classs

- [DependencyKind](DependencyKind.md) — The kind of EDA dependency being tracked.
- [Document](Document.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [DocumentType](DocumentType.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
- [GitReference](GitReference.md) — Target Git reference or constraint for resolving a dependency.
- [GitSource](GitSource.md) — Git repository source specification.
- [LibraryEntry](LibraryEntry.md) — One library reference recorded in `.snxprj`. The project loader
- [LibraryEntryKind](LibraryEntryKind.md) — How a [`LibraryEntry`] resolves on disk. Project-local libraries live
- [LockedDependency](LockedDependency.md) — A locked, exact resolution record stored in `project.lock`.
- [ProjectData](ProjectData.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ProjectDependency](ProjectDependency.md) — A versioned dependency declared in `.snxprj`.
- [ProjectError](ProjectError.md) — [derive(Debug, Error)]
- [ProjectLockfile](ProjectLockfile.md) — Deterministic lockfile format (`project.lock`) for reproducible EDA designs.
- [SheetEntry](SheetEntry.md) — [derive(Debug, Clone, Serialize, Deserialize)]

## Functions

- [default](default.md)
- [default](default_1.md)
- [default_true](default_true.md)
- [has_stray_tmp](has_stray_tmp.md) — True if `dir` contains a leftover atomic-write temp sibling
- [library_entry_loads_without_library_id_field](library_entry_loads_without_library_id_field.md) — [test]
- [library_entry_round_trips_through_serde](library_entry_round_trips_through_serde.md) — [test]
- [parse_project](parse_project.md) — Parse a `.snxprj` project file. Two paths:
- [parse_project_errors_on_corrupt_json_instead_of_degrading](parse_project_errors_on_corrupt_json_instead_of_degrading.md) — [test]
- [parse_project_probes_directory_for_non_json_marker](parse_project_probes_directory_for_non_json_marker.md) — [test]
- [project_data_loads_without_libraries_field](project_data_loads_without_libraries_field.md) — `.snxprj` files written before the `libraries` field landed
- [project_dependencies_and_lockfile_round_trip](project_dependencies_and_lockfile_round_trip.md) — [test]
- [resolve_library_path](resolve_library_path.md) — Resolve a [`LibraryEntry`]'s `path` to an absolute path. Project-
- [resolve_library_path](resolve_library_path_1.md) — Resolve a [`LibraryEntry`]'s `path` to an absolute path. Project-
- [resolve_library_path_joins_project_local](resolve_library_path_joins_project_local.md) — [test]
- [resolve_library_path_keeps_absolute_for_shared](resolve_library_path_keeps_absolute_for_shared.md) — [test]
- [write_project](write_project.md) — Serialize `data` to `path` as pretty JSON. Companion of
- [write_project_round_trips_atomically_leaving_no_tmp](write_project_round_trips_atomically_leaving_no_tmp.md) — [test]
