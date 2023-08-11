pub mod command;
pub mod command_palette;
pub mod icons;

struct AppState {
    left_panel: bool,
    right_panel: bool,
    bottom_panel: bool,
    // cmd_palette: CommandPalette,
    // pub command_sender: CommandSender,
    // command_receiver: CommandReceiver,
}

fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}
