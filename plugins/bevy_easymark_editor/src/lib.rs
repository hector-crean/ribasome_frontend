pub mod easy_mark_editor;
pub mod easy_mark_highlighter;
pub mod easy_mark_parser;
pub mod easy_mark_viewer;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use easy_mark_editor::EasyMarkEditor;

pub struct EasyMarkEditorPlugin;

impl Plugin for EasyMarkEditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EasyMarkEditor::default())
            .add_systems(Update, Self::update);
    }
}

impl EasyMarkEditorPlugin {
    pub fn update(mut easy_mark_editor: ResMut<EasyMarkEditor>, mut egui_ctx: EguiContexts) {
        let ctx = egui_ctx.ctx_mut();

        egui::SidePanel::right("easy mark side panel")
            .min_width(150.0)
            .default_width(180.0)
            .show(ctx, |ui| easy_mark_editor.ui(ui));
    }
}
