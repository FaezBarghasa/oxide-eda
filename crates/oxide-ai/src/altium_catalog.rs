//! Shared Altium Designer `.rcs` UI catalog index (3,004 commands across 39 `.rcs` files).
//!
//! Exposes fast search, namespace statistics, and command inspection for both
//! the MCP/UTCP server (`oxide-ai`) and the rich CLI (`oxide-cli`).

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AltiumUiCommand {
    pub command: String,
    pub caption: String,
    pub image: String,
    pub shortcut1: String,
    pub shortcut2: String,
    pub params: String,
    pub description: String,
    pub object: String,
}

impl AltiumUiCommand {
    pub fn namespace(&self) -> &str {
        match self.command.find(':') {
            Some(idx) => &self.command[..idx],
            None => "@interactive",
        }
    }

    pub fn action(&self) -> &str {
        match self.command.find(':') {
            Some(idx) => &self.command[idx + 1..],
            None => &self.command,
        }
    }

    pub fn plain_caption(&self) -> String {
        self.caption.replace('&', "")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AltiumCatalogEntry {
    pub source_file: String,
    #[serde(flatten)]
    pub cmd: AltiumUiCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AltiumCatalogStats {
    pub total_entries: usize,
    pub unique_commands: usize,
    pub rcs_file_count: usize,
    pub namespace_count: usize,
    pub top_rcs_files: Vec<(String, usize)>,
    pub top_namespaces: Vec<(String, usize)>,
}

pub struct AltiumCatalog {
    entries: Vec<AltiumCatalogEntry>,
    by_command: HashMap<String, Vec<usize>>,
    by_file: BTreeMap<String, usize>,
    by_namespace: BTreeMap<String, usize>,
}

static CATALOG: LazyLock<AltiumCatalog> = LazyLock::new(AltiumCatalog::load);

impl AltiumCatalog {
    pub fn global() -> &'static Self {
        &CATALOG
    }

    fn load() -> Self {
        let json =
            include_str!("../../../refernces/AltiumDesigner/decompiled/altium_ui_catalog.json");
        let raw: HashMap<String, Vec<AltiumUiCommand>> = serde_json::from_str(json)
            .unwrap_or_else(|e| panic!("failed to parse altium_ui_catalog.json: {e}"));

        let mut entries = Vec::new();
        let mut by_command: HashMap<String, Vec<usize>> = HashMap::new();
        let mut by_file: BTreeMap<String, usize> = BTreeMap::new();
        let mut by_namespace: BTreeMap<String, usize> = BTreeMap::new();

        let mut sorted_files: Vec<(String, Vec<AltiumUiCommand>)> = raw.into_iter().collect();
        sorted_files.sort_by(|a, b| a.0.cmp(&b.0));

        for (source_file, cmds) in sorted_files {
            by_file.insert(source_file.clone(), cmds.len());
            for cmd in cmds {
                let idx = entries.len();
                *by_namespace
                    .entry(cmd.namespace().to_string())
                    .or_insert(0) += 1;
                by_command
                    .entry(cmd.command.clone())
                    .or_default()
                    .push(idx);
                entries.push(AltiumCatalogEntry { source_file: source_file.clone(), cmd });
            }
        }

        Self {
            entries,
            by_command,
            by_file,
            by_namespace,
        }
    }

    pub fn stats(&self) -> AltiumCatalogStats {
        let mut top_rcs_files: Vec<(String, usize)> =
            self.by_file.iter().map(|(k, v)| (k.clone(), *v)).collect();
        top_rcs_files.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        let mut top_namespaces: Vec<(String, usize)> = self
            .by_namespace
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        top_namespaces.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        AltiumCatalogStats {
            total_entries: self.entries.len(),
            unique_commands: self.by_command.len(),
            rcs_file_count: self.by_file.len(),
            namespace_count: self.by_namespace.len(),
            top_rcs_files,
            top_namespaces,
        }
    }

    pub fn inspect(&self, command: &str) -> Vec<&AltiumCatalogEntry> {
        if let Some(indices) = self.by_command.get(command) {
            return indices.iter().map(|&i| &self.entries[i]).collect();
        }
        let needle = command.to_ascii_lowercase();
        self.entries
            .iter()
            .filter(|e| e.cmd.command.to_ascii_lowercase() == needle)
            .collect()
    }

    pub fn search(
        &self,
        query: &str,
        namespace: Option<&str>,
        rcs_file: Option<&str>,
        limit: usize,
    ) -> Vec<&AltiumCatalogEntry> {
        let q = query.trim().to_ascii_lowercase();
        let ns_filter = namespace.map(|s| s.trim().to_ascii_lowercase());
        let file_filter = rcs_file.map(|s| s.trim().to_ascii_lowercase());
        let cap = limit.clamp(1, 500);

        let mut scored: Vec<(u8, &AltiumCatalogEntry)> = Vec::new();
        let mut seen_cmds = HashSet::new();

        for entry in &self.entries {
            if let Some(ref ns) = ns_filter
                && !entry.cmd.namespace().to_ascii_lowercase().contains(ns)
            {
                continue;
            }
            if let Some(ref rf) = file_filter
                && !entry.source_file.to_ascii_lowercase().contains(rf)
            {
                continue;
            }

            if q.is_empty() {
                if seen_cmds.insert((&entry.cmd.command, &entry.source_file)) {
                    scored.push((3, entry));
                }
                continue;
            }

            let cmd_l = entry.cmd.command.to_ascii_lowercase();
            let cap_l = entry.cmd.plain_caption().to_ascii_lowercase();
            let desc_l = entry.cmd.description.to_ascii_lowercase();
            let sc1_l = entry.cmd.shortcut1.to_ascii_lowercase();

            let rank = if cmd_l == q {
                Some(0)
            } else if cmd_l.starts_with(&q) || cap_l.starts_with(&q) {
                Some(1)
            } else if cmd_l.contains(&q) || cap_l.contains(&q) {
                Some(2)
            } else if desc_l.contains(&q) || sc1_l.contains(&q) {
                Some(3)
            } else {
                None
            };

            if let Some(r) = rank
                && seen_cmds.insert((&entry.cmd.command, &entry.source_file))
            {
                scored.push((r, entry));
            }
        }

        scored.sort_by(|a, b| {
            a.0.cmp(&b.0)
                .then_with(|| a.1.cmd.command.cmp(&b.1.cmd.command))
        });
        scored.into_iter().take(cap).map(|(_, e)| e).collect()
    }
}
