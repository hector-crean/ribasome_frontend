pub mod command;
pub mod cursor;
pub mod editor;
pub mod formatting;
pub mod highlighter;
pub mod parser;
pub mod text_buffer;
pub mod viewer;

use std::sync::Arc;

use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self,
        epaint::{
            text::{LayoutJob, TextFormat},
            Color32, FontFamily, FontId,
        },
    },
    EguiContexts,
};
use peritext::RichText as RichTextBufferInner;

use bevy::prelude::*;

pub struct RichTextEditorPlugin;

impl Plugin for RichTextEditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RichTextEditor::default())
            .add_systems(Update, Self::update);
    }
}

impl RichTextEditorPlugin {
    pub fn update(mut rich_text_editor: ResMut<RichTextEditor>, mut egui_ctx: EguiContexts) {
        let ctx = egui_ctx.ctx_mut();

        egui::SidePanel::right("rte side panel")
            .min_width(150.0)
            .default_width(180.0)
            .show(ctx, |ui| {
                rich_text_editor.ui(ui);
            });
    }
}

pub struct RichTextBuffer(RichTextBufferInner);

// Implement the Deref trait to allow implicit dereferencing
// use std::ops::Deref;
// impl Deref for RichText {
//     type Target = RichTextBufferInner;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// Implement From and Into traits for easy conversion
impl From<RichTextBufferInner> for RichTextBuffer {
    fn from(buffer: RichTextBufferInner) -> Self {
        RichTextBuffer(buffer)
    }
}

impl From<RichTextBuffer> for RichTextBufferInner {
    fn from(rich_text_buffer: RichTextBuffer) -> Self {
        rich_text_buffer.0
    }
}

impl RichTextBuffer {
    pub fn new(buffer: RichTextBufferInner) -> Self {
        RichTextBuffer(buffer)
    }
}

///
impl egui::TextBuffer for RichTextBuffer {
    fn as_str(&self) -> &str {
        todo!()
    }
    fn byte_index_from_char_index(&self, char_index: usize) -> usize {
        todo!()
    }
    fn char_range(&self, char_range: std::ops::Range<usize>) -> &str {
        todo!()
    }

    fn clear(&mut self) {
        self.0.delete(..)
    }
    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.0.delete(char_range)
    }
    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.0.insert(char_index, text);
        text.chars().count()
    }
    fn is_mutable(&self) -> bool {
        true
    }
    fn replace(&mut self, text: &str) {
        todo!()
    }
    fn take(&mut self) -> String {
        todo!()
    }
}

#[derive(Resource)]
pub struct RichTextEditor {
    rich_text: String,
}

impl RichTextEditor {
    fn new(initial_text: String) -> Self {
        Self {
            rich_text: initial_text,
        }
    }
}
impl Default for RichTextEditor {
    fn default() -> Self {
        Self {
            rich_text: "".into(),
        }
    }
}

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

impl View for RichTextEditor {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Advanced usage of ");
            ui.code("TextEdit");
            ui.label(".");
        });

        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut job = LayoutJob::default();
            job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(job))
        };

        let text_buffer = egui::TextEdit::multiline(&mut self.rich_text)
            .hint_text("Type something!")
            .font(egui::TextStyle::Monospace) // for cursor height
            .code_editor()
            .desired_rows(10)
            .lock_focus(true)
            .desired_width(f32::INFINITY)
            .layouter(&mut layouter);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(text_buffer);
        });

        // ui.horizontal(|ui| {
        //     ui.spacing_mut().item_spacing.x = 0.0;
        //     ui.label("Selected text: ");
        //     if let Some(text_cursor_range) = output.cursor_range {
        //         use egui::TextBuffer as _;
        //         let selected_chars = text_cursor_range.as_sorted_char_range();
        //         let selected_text = self.rich_text.char_range(selected_chars);
        //         ui.code(selected_text);
        //     }
        // });
    }
}

// fn show_example_multiline_hover(ui: &mut egui::Ui) {
//     let mut text = "Hello world!\nI can do tooltips!\nPretty neat.";

//     let text_edit_output = egui::TextEdit::multiline(&mut text).show(ui);
//     let hover_pos = ui.input().pointer.hover_pos();
//     if let Some(hover_pos) = hover_pos {
//         if text_edit_output.response.rect.contains(hover_pos) {
//             let hover_pos = hover_pos - text_edit_output.response.rect.left_top();
//             let hover_cursor = text_edit_output.galley.cursor_from_pos(hover_pos).pcursor;
//             if let Some(line) = text.lines().nth(hover_cursor.paragraph) {
//                 egui::show_tooltip_at_pointer(ui.ctx(), egui::Id::new("hover tooltip"), |ui| {
//                     ui.label(line);
//                 });
//             }
//         }
//     }
// }
