use ribasome_state::tool::ToolState;
use std::{collections::BTreeSet, default};

use bevy_egui::{
    egui,
    egui::{Align2, Key, NumExt as _},
    EguiContext, EguiContexts, EguiPlugin,
};

use crate::command::CommandMsg;

use bevy::prelude::*;

pub struct CommandPalettePlugin;

impl Plugin for CommandPalettePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CommandPalette::default())
            .add_event::<CommandMsg>()
            .add_systems(
                Update,
                (toggle_cmd_palette, cmd_emitter, cmd_consumer).chain(),
            );
    }
}

#[derive(Resource)]
pub struct CommandPalette {
    visible: bool,
    query: String,
    selected_alternative: usize,
}
impl Default for CommandPalette {
    fn default() -> Self {
        Self {
            visible: false,
            query: "".to_string(),
            selected_alternative: 0,
        }
    }
}

fn toggle_cmd_palette(kbd_input: Res<Input<KeyCode>>, mut command_palette: ResMut<CommandPalette>) {
    let shift = kbd_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let ctrl = kbd_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let meta = kbd_input.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);

    if (ctrl || shift || meta) && kbd_input.just_pressed(KeyCode::P) {
        command_palette.toggle()
    }
}

fn cmd_emitter(
    mut egui_ctx: EguiContexts,
    mut command_palette: ResMut<CommandPalette>,
    mut cmd_writer: EventWriter<CommandMsg>,
) {
    let cmd: Option<CommandMsg> = command_palette.run(egui_ctx.ctx_mut());

    match cmd {
        Some(cmd) => {
            cmd_writer.send(cmd);
        }
        None => {}
    }
}

fn cmd_consumer(
    mut cmd_event: EventReader<CommandMsg>,
    mut command_palette: ResMut<CommandPalette>,
    mut next_tool_state: ResMut<NextState<ToolState>>,
) {
    for cmd in cmd_event.iter() {
        match cmd {
            CommandMsg::AddTool => next_tool_state.set(ToolState::Add),
            CommandMsg::EditTool => next_tool_state.set(ToolState::Edit),
            CommandMsg::TexturePaintTool => next_tool_state.set(ToolState::TexturePaint),
            CommandMsg::TransformTool => next_tool_state.set(ToolState::Transform),
            CommandMsg::VertexPaintTool => next_tool_state.set(ToolState::VertexPaint),
        }
    }
}

impl CommandPalette {
    pub fn toggle(&mut self) {
        self.visible ^= true;
    }

    /// Show the command palette, if it is visible.
    #[must_use = "Returns the command that was selected"]
    pub fn run(&mut self, egui_ctx: &egui::Context) -> Option<CommandMsg> {
        self.visible &= !egui_ctx.input_mut(|i| i.consume_key(Default::default(), Key::Escape));
        if !self.visible {
            self.query.clear();
            return None;
        }

        let screen_rect = egui_ctx.screen_rect();
        let width = 300.0;
        let max_height = 320.0.at_most(screen_rect.height());

        let cmd = egui::Window::new("Command Palette")
            .title_bar(false)
            .fixed_size([width, max_height])
            .pivot(egui::Align2::CENTER_TOP)
            .fixed_pos(screen_rect.center() - 0.5 * max_height * egui::Vec2::Y)
            .show(egui_ctx, |ui| self.select_command_ui(ui))?
            .inner?;

        cmd
    }

    #[must_use = "Returns the command that was selected"]
    fn select_command_ui(&mut self, ui: &mut egui::Ui) -> Option<CommandMsg> {
        // Check _before_ we add the `TextEdit`, so it doesn't steal it.
        let enter_pressed = ui.input_mut(|i| i.consume_key(Default::default(), Key::Enter));

        let text_response = ui.add(
            egui::TextEdit::singleline(&mut self.query)
                .desired_width(f32::INFINITY)
                .lock_focus(true),
        );
        text_response.request_focus();
        let mut scroll_to_selected_alternative = false;
        if text_response.changed() {
            self.selected_alternative = 0;
            scroll_to_selected_alternative = true;
        }

        let selected_command = egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                self.alternatives_ui(ui, enter_pressed, scroll_to_selected_alternative)
            })
            .inner;

        if selected_command.is_some() {
            *self = Default::default();
        }

        selected_command
    }

    #[must_use = "Returns the command that was selected"]
    fn alternatives_ui(
        &mut self,
        ui: &mut egui::Ui,
        enter_pressed: bool,
        mut scroll_to_selected_alternative: bool,
    ) -> Option<CommandMsg> {
        scroll_to_selected_alternative |= ui.input(|i| i.key_pressed(Key::ArrowUp));
        scroll_to_selected_alternative |= ui.input(|i| i.key_pressed(Key::ArrowDown));

        let query = self.query.to_lowercase();

        let item_height = 16.0;
        let font_id = egui::TextStyle::Button.resolve(ui.style());

        let mut num_alternatives: usize = 0;
        let mut selected_command = None;

        for (i, fuzzy_match) in commands_that_match(&query).iter().enumerate() {
            let command = fuzzy_match.command;
            let kb_shortcut = command
                .kb_shortcut()
                .map(|shortcut| ui.ctx().format_shortcut(&shortcut))
                .unwrap_or_default();

            let (rect, response) = ui.allocate_at_least(
                egui::vec2(ui.available_width(), item_height),
                egui::Sense::click(),
            );

            // let response = response.on_hover_text(command.tooltip());

            if response.clicked() {
                selected_command = Some(command);
            }

            let selected = i == self.selected_alternative;
            let style = ui.style().interact_selectable(&response, selected);

            if selected {
                ui.painter()
                    .rect_filled(rect, style.rounding, ui.visuals().selection.bg_fill);

                if enter_pressed {
                    selected_command = Some(command);
                }

                if scroll_to_selected_alternative {
                    ui.scroll_to_rect(rect, None);
                }
            }

            let text = format_match(fuzzy_match, ui, &font_id, style.text_color());

            // TODO(emilk): shorten long text using '…'
            let galley = text
                .into_galley(
                    ui,
                    Some(false),
                    f32::INFINITY,
                    egui::FontSelection::default(),
                )
                .galley;
            let text_rect = Align2::LEFT_CENTER
                .anchor_rect(egui::Rect::from_min_size(rect.left_center(), galley.size()));
            ui.painter().galley(text_rect.min, galley);

            ui.painter().text(
                rect.right_center(),
                Align2::RIGHT_CENTER,
                kb_shortcut,
                font_id.clone(),
                if selected {
                    style.text_color()
                } else {
                    ui.visuals().weak_text_color()
                },
            );

            num_alternatives += 1;
        }

        if num_alternatives == 0 {
            ui.weak("No matching results");
        }

        // Move up/down in the list:
        self.selected_alternative = self.selected_alternative.saturating_sub(
            ui.input_mut(|i| i.count_and_consume_key(Default::default(), Key::ArrowUp)),
        );
        self.selected_alternative = self.selected_alternative.saturating_add(
            ui.input_mut(|i| i.count_and_consume_key(Default::default(), Key::ArrowDown)),
        );

        self.selected_alternative = self
            .selected_alternative
            .clamp(0, num_alternatives.saturating_sub(1));

        selected_command
    }
}

struct FuzzyMatch {
    command: CommandMsg,
    score: isize,
    fuzzy_match: Option<sublime_fuzzy::Match>,
}

fn commands_that_match(query: &str) -> Vec<FuzzyMatch> {
    use strum::IntoEnumIterator as _;

    if query.is_empty() {
        CommandMsg::iter()
            .map(|command| FuzzyMatch {
                command,
                score: 0,
                fuzzy_match: None,
            })
            .collect()
    } else {
        let mut matches: Vec<_> = CommandMsg::iter()
            .filter_map(|command| {
                let target_text = command.str();
                sublime_fuzzy::best_match(query, target_text).map(|fuzzy_match| FuzzyMatch {
                    command,
                    score: fuzzy_match.score(),
                    fuzzy_match: Some(fuzzy_match),
                })
            })
            .collect();
        matches.sort_by_key(|m| -m.score);
        matches
    }
}

fn format_match(
    m: &FuzzyMatch,
    ui: &egui::Ui,
    font_id: &egui::FontId,
    default_text_color: egui::Color32,
) -> egui::WidgetText {
    let target_text = m.command.str();

    if let Some(fm) = &m.fuzzy_match {
        let matched_indices: BTreeSet<_> = fm.matched_indices().collect();

        let mut job = egui::text::LayoutJob::default();
        for (i, c) in target_text.chars().enumerate() {
            let color = if matched_indices.contains(&i) {
                ui.visuals().strong_text_color()
            } else {
                default_text_color
            };
            job.append(
                &c.to_string(),
                0.0,
                egui::text::TextFormat::simple(font_id.clone(), color),
            );
        }

        job.into()
    } else {
        egui::RichText::new(target_text)
            .color(default_text_color)
            .into()
    }
}
