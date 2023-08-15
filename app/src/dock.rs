use bevy::prelude::World;
use bevy_egui::egui;
use egui_dock::{DockArea, NodeIndex, Style, Tree};

#[derive(Debug)]
pub enum Tab {
    GameView,
    Hierarchy,
    Resources,
    Assets,
    Inspector,
}

pub struct Tahs {
    tree: Tree<Tab>,
}

impl Tahs {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![Tab::GameView]);
        let [game, _inspector] = tree.split_right(NodeIndex::root(), 0.75, vec![Tab::Inspector]);
        let [game, _hierarchy] = tree.split_left(game, 0.2, vec![Tab::Hierarchy]);
        let [_game, _bottom] = tree.split_below(game, 0.8, vec![Tab::Resources, Tab::Assets]);

        Self { tree }
    }

    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer { world };
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }
}

pub struct TabViewer<'a> {
    world: &'a mut World,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = Tab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            _ => {}
        }
    }
    fn title(&mut self, tab: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{tab:?}").into()
    }
}
