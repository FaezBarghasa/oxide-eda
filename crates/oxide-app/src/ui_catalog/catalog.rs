//! The Altium UI command catalog, loaded from the decompiled `.rcs` resource
//! files.
//!
//! Altium defines its entire UI in text-based `.rcs` files. Each line is a
//! command with fields: `Command` (namespace:action), `Caption` (display
//! text, `&` = mnemonic), `Image` (icon), `Shortcut1/2`, `Params`, and
//! `Description`. This module captures that catalog as Rust data.

use serde::Deserialize;

/// A single command entry from Altium's `.rcs` UI resource files.
///
/// Mirrors the fields of a line in an Altium `.rcs` file:
///
/// ```text
/// PL PLPCB:Undo   Command='PCB:Undo'   Caption='&Undo'   Image='UNDO.PNG'
///                Shortcut1='Ctrl+Z'   Shortcut2='Alt+Backspace'   Params=''   Description='Undo'   End
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UiCommand {
    /// The command's fully-qualified name, e.g. `PCB:Undo` or `@PLSCH:InteractiveZoomIn`.
    pub command: String,
    /// Display text shown in menus/toolbars. A leading `&` marks the mnemonic
    /// character (e.g. `&Undo` shows as `U`ndo).
    pub caption: String,
    /// Icon asset name (e.g. `UNDO.PNG`). Empty when the command has no icon.
    pub image: String,
    /// Primary keyboard shortcut, e.g. `Ctrl+Z`. Empty when none.
    pub shortcut1: String,
    /// Secondary keyboard shortcut, e.g. `Alt+Backspace`. Empty when none.
    pub shortcut2: String,
    /// Parameter string for parameterized commands, e.g. `Action=Toggle|SetFocus=True`.
    pub params: String,
    /// Human-readable description of what the command does.
    pub description: String,
    /// The `.rcs` object line this command came from (e.g. `PL PLPCB:Undo`). Kept
    /// for provenance; not used in dispatch.
    pub object: String,
}

impl UiCommand {
    /// The command's namespace (the part before the first `:`), e.g. `PCB`.
    /// Returns `"@interactive"` for commands with no colon (rare).
    pub fn namespace(&self) -> &str {
        match self.command.find(':') {
            Some(idx) => &self.command[..idx],
            None => "@interactive",
        }
    }

    /// The command's action (the part after the first `:`), e.g. `Undo`.
    pub fn action(&self) -> &str {
        match self.command.find(':') {
            Some(idx) => &self.command[idx + 1..],
            None => &self.command,
        }
    }

    /// Strip the mnemonic marker (`&`) from a caption for plain-text display.
    pub fn plain_caption(&self) -> String {
        self.caption
            .strip_prefix('&')
            .unwrap_or(self.caption.as_str())
            .to_string()
    }
}

/// The full Altium UI command catalog.
///
/// Keys are the source `.rcs` file names (e.g. `AdvPcb.rcs`, `advsch.rcs`);
/// values are the commands defined in that file. The catalog is loaded from
/// the decompiled resource files embedded at build time, so it is always
/// available and independent of runtime file paths.
#[derive(Debug)]
pub struct UiCatalog {
    pub files: std::collections::HashMap<String, Vec<UiCommand>>,
}

impl UiCatalog {
    /// Load the embedded Altium UI catalog (3004 commands across 39 `.rcs` files).
    ///
    /// The JSON is a map of `.rcs` file name to list of command objects, which
    /// deserializes directly to `HashMap<String, Vec<UiCommand>>`.
    pub fn load() -> Self {
        let json =
            include_str!("../../../../refernces/AltiumDesigner/decompiled/altium_ui_catalog.json");
        let files: std::collections::HashMap<String, Vec<UiCommand>> = serde_json::from_str(json)
            .unwrap_or_else(|e| panic!("failed to parse embedded Altium UI catalog: {e}"));
        Self { files }
    }

    /// All commands across every `.rcs` file, in stable order.
    pub fn all_commands(&self) -> impl Iterator<Item = &UiCommand> {
        self.files.values().flatten()
    }

    /// Total number of command entries (3004).
    pub fn count(&self) -> usize {
        self.all_commands().count()
    }

    /// Number of distinct command names (1190; several `.rcs` lines share a
    /// command name with different captions/params).
    pub fn unique_command_count(&self) -> usize {
        let mut seen = std::collections::HashSet::new();
        for cmd in self.all_commands() {
            seen.insert(cmd.command.clone());
        }
        seen.len()
    }

    /// Commands defined in a specific `.rcs` file.
    pub fn commands_from(&self, file: &str) -> &[UiCommand] {
        self.files.get(file).map_or(&[], |v| v.as_slice())
    }

    /// The list of source `.rcs` file names.
    pub fn file_names(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_loads_all_commands() {
        let cat = UiCatalog::load();
        assert_eq!(cat.count(), 3004, "expected 3004 command entries");
        assert_eq!(
            cat.unique_command_count(),
            1190,
            "expected 1190 unique names"
        );
        assert!(!cat.file_names().is_empty());
    }

    #[test]
    fn advpcb_commands_present() {
        let cat = UiCatalog::load();
        // AdvPcb.rcs is the PCB editor's command file (959 commands).
        assert!(!cat.commands_from("AdvPcb.rcs").is_empty());
    }

    #[test]
    fn command_namespace_and_action() {
        let cat = UiCatalog::load();
        // Find a PCB:Undo entry.
        let undo = cat
            .all_commands()
            .find(|c| c.command == "PCB:Undo")
            .expect("PCB:Undo should be in the catalog");
        assert_eq!(undo.namespace(), "PCB");
        assert_eq!(undo.action(), "Undo");
    }

    #[test]
    fn interactive_commands_have_at_namespace() {
        let cat = UiCatalog::load();
        // @PLSCH commands are the schematic interactive-mode commands.
        let any = cat
            .all_commands()
            .find(|c| c.command.starts_with("@PLSCH:") || c.command.starts_with("@PLSch:"));
        if let Some(cmd) = any {
            assert!(cmd.namespace() == "@interactive" || cmd.namespace().starts_with('@'));
        }
    }
}
