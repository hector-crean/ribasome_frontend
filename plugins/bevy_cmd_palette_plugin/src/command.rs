use bevy::prelude::*;
use bevy_egui::{
    egui,
    egui::{Key, KeyboardShortcut, Modifiers},
};

use strum::{EnumMessage, IntoEnumIterator};
// use strum_macros::{Display, EnumIter, EnumMessage, EnumString, IntoStaticStr};

/// Interface for sending [`CommandMsg`] messages.
pub trait CommandMsgSender {
    fn send_ui(&self, command: CommandMsg);
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
pub enum CommandMsg {
    // Listed in the order they show up in the command palette by default!
    #[strum(serialize = "transform_tool")]
    #[strum(message = "Transform")]
    TransformTool,
    #[strum(serialize = "edit")]
    #[strum(message = "Edit")]
    EditTool,
    #[strum(serialize = "add")]
    #[strum(message = "Add")]
    AddTool,
    #[strum(serialize = "vetex_paint")]
    #[strum(message = "Vertex Paint")]
    VertexPaintTool,
    #[strum(serialize = "texture_paint")]
    #[strum(message = "Texture Paint")]
    TexturePaintTool,
}

impl CommandMsg {
    pub fn desc(self) -> Option<&'static str> {
        self.get_message()
    }
    pub fn str(&self) -> &'static str {
        self.into()
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
            CommandMsg::TransformTool => Some(cmd(Key::T)),
            CommandMsg::EditTool => Some(cmd(Key::E)),
            CommandMsg::AddTool => Some(cmd(Key::A)),
            CommandMsg::VertexPaintTool => Some(cmd(Key::V)),
            CommandMsg::TexturePaintTool => Some(cmd(Key::W)),
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

    for a_cmd in CommandMsg::iter() {
        if let Some(a_shortcut) = a_cmd.kb_shortcut() {
            for b_cmd in CommandMsg::iter() {
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
