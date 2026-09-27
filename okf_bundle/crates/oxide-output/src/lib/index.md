# lib

## Classs

- [ExportContext](ExportContext.md) — Everything an exporter needs to know about the project being exported.
- [Exporter](Exporter.md) — The universal exporter trait — one impl per output format.
- [ExportError](ExportError.md) — [derive(Debug, Error)]
- [ProjectMetadata](ProjectMetadata.md) — Title-block / project-file metadata used to resolve `${TITLE}`, `${REV}`,
- [SheetSnapshot](SheetSnapshot.md) — [derive(Debug, Clone)]
