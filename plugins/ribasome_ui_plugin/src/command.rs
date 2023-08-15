use bevy::prelude::*;
use bevy_egui::{
    egui,
    egui::{Key, KeyboardShortcut, Modifiers},
};

use strum::{EnumMessage, IntoEnumIterator};
// use strum_macros::{Display, EnumIter, EnumMessage, EnumString, IntoStaticStr};

/// Interface for sending [`UICommand`] messages.
pub trait UICommandSender {
    fn send_ui(&self, command: UICommand);
}

/// All the commands we support.
///
/// Most are available in the GUI,
/// some have keyboard shortcuts,
/// and all are visible in the [`crate::CommandPalette`].
///

#[derive(
    Event,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::EnumIter,
    Clone,
    PartialEq,
    Eq,
    Debug,
    strum_macros::EnumMessage,
    strum_macros::IntoStaticStr,
    Copy,
)]
pub enum UICommand {
    // Listed in the order they show up in the command palette by default!
    #[strum(serialize = "open")]
    #[strum(message = "Open")]
    Open,
    #[strum(serialize = "save")]
    #[strum(message = "Save")]
    Save,
    #[strum(serialize = "toggle_command_palette")]
    #[strum(message = "Toggle Command Palette")]
    ToggleCommandPalette,
}

impl UICommand {
    pub fn desc(self) -> Option<&'static str> {
        match &self {
            UICommand::Open => self.get_message(),
            UICommand::Save => self.get_message(),
            UICommand::ToggleCommandPalette => self.get_message(),
        }
    }
    pub fn str(&self) -> &'static str {
        match &self {
            UICommand::Open => self.into(),
            UICommand::Save => self.into(),
            UICommand::ToggleCommandPalette => self.into(),
        }
    }

    #[allow(clippy::unnecessary_wraps)] // Only on some platforms
    pub fn kb_shortcut(self) -> Option<KeyboardShortcut> {
        fn key(key: Key) -> KeyboardShortcut {
            KeyboardShortcut::new(Modifiers::NONE, key)
        }

        fn cmd(key: Key) -> KeyboardShortcut {
            KeyboardShortcut::new(Modifiers::COMMAND, key)
        }

        fn cmd_alt(key: Key) -> KeyboardShortcut {
            KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::ALT), key)
        }

        fn ctrl_shift(key: Key) -> KeyboardShortcut {
            KeyboardShortcut::new(Modifiers::CTRL.plus(Modifiers::SHIFT), key)
        }

        match self {
            UICommand::Save => Some(cmd(Key::S)),
            UICommand::Open => Some(cmd(Key::O)),
            UICommand::ToggleCommandPalette => Some(cmd(Key::P)),
        }
    }

    /// Add e.g. " (Ctrl+F11)" as a suffix
    pub fn format_shortcut_tooltip_suffix(self, egui_ctx: &egui::Context) -> String {
        if let Some(kb_shortcut) = self.kb_shortcut() {
            format!(" ({})", egui_ctx.format_shortcut(&kb_shortcut))
        } else {
            Default::default()
        }
    }
}

#[test]
fn check_for_clashing_command_shortcuts() {
    fn clashes(a: KeyboardShortcut, b: KeyboardShortcut) -> bool {
        if a.key != b.key {
            return false;
        }

        if a.modifiers.alt != b.modifiers.alt {
            return false;
        }

        if a.modifiers.shift != b.modifiers.shift {
            return false;
        }

        // On Non-Mac, command is interpreted as ctrl!
        (a.modifiers.command || a.modifiers.ctrl) == (b.modifiers.command || b.modifiers.ctrl)
    }

    use strum::IntoEnumIterator as _;

    for a_cmd in UICommand::iter() {
        if let Some(a_shortcut) = a_cmd.kb_shortcut() {
            for b_cmd in UICommand::iter() {
                if a_cmd == b_cmd {
                    continue;
                }
                if let Some(b_shortcut) = b_cmd.kb_shortcut() {
                    assert!(
                        !clashes(a_shortcut, b_shortcut),
                        "Command '{a_cmd:?}' and '{b_cmd:?}' have overlapping keyboard shortcuts: {:?} vs {:?}",
                        a_shortcut.format(&egui::ModifierNames::NAMES, true),
                        b_shortcut.format(&egui::ModifierNames::NAMES, true),
                    );
                }
            }
        }
    }
}
