pub mod canvas;
pub mod coordinate_system;
pub mod dock;
pub mod drag_and_drop;
pub mod errors;
pub mod light_rig;
pub mod marker_component;
pub mod state;
pub mod ui;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_add_tool::AddToolPlugin;
use bevy_asset_loader::prelude::*;
use bevy_cameras::pan_orbit_camera::OrbitCameraController;
use bevy_cmd_palette_plugin::CommandPalettePlugin;
use bevy_drag::RaycastPickCamera;
use bevy_easymark_editor::EasyMarkEditorPlugin;
use bevy_editor::EditorPlugin;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_outline::*;
use bevy_mod_picking::{
    prelude::{
        low_latency_window_plugin, DebugPickingPlugin, DefaultHighlightingPlugin, EguiBackend,
    },
    DefaultPickingPlugins,
};
use bevy_mod_reqwest::*;
use bevy_rich_text_editor::RichTextEditorPlugin;

use dock::TabViewer;
use drag_and_drop::{file_drag_and_drop, FileDragAndDropPlugin};
use egui_dock::{DockArea, NodeIndex, Style, Tree};
use ribasome_state::tool::ToolState;

use canvas::Canvas3dPlugin;
use light_rig::LightRigPlugin;
use ribasome_models::marker_3d::Marker3d;
use ribasome_state::marker::MainCamera;
use ui::UiShellPlugin;
const BILLBOARD_TEXT_SCALE: Vec3 = Vec3::splat(0.0085);

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/iosevka-regular.ttf")]
    pub iosevka_regular: Handle<Font>,
    #[asset(path = "fonts/iosevka-heavy.ttf")]
    pub iosevka_heavy: Handle<Font>,
    #[asset(path = "fonts/iosevka-heavyitalic.ttf")]
    pub iosevka_heavyitalic: Handle<Font>,
    #[asset(path = "fonts/iosevka-extraboldoblique.ttf")]
    pub iosevka_extraboldoblique: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct GlbAssets {
    // #[asset(path = "glb/AF-Q8W3K0-F1.glb#Scene0")]
    // pub AF_Q8W3K0_F1: Handle<Scene>,
}

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Resource, Debug, Component)]
struct Labels(Vec<Marker3d>);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Menu,
    Canvas3d,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum Tool {
    Pen,
    Labeller,
}

pub struct AppPlugin;

impl AppPlugin {
    fn setup(mut commands: Commands) {
        commands
            .spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0., 0., -2.).looking_at(Vec3::ZERO, Vec3::Y),
                    // projection: Projection::Orthographic(OrthographicProjection::default()),
                    ..default()
                },
                MainCamera,
                OrbitCameraController::default(),
            ))
            .insert(RaycastPickCamera::default());
    }
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>()
                .enable::<EguiBackend>(),
            EguiPlugin,
            OutlinePlugin,
            UiShellPlugin,
            Canvas3dPlugin,
            CommandPalettePlugin,
            FileDragAndDropPlugin,
            LightRigPlugin,
            AddToolPlugin::<ToolState, 2> {
                run_states: [ToolState::Add, ToolState::Edit],
                on_exit_state: ToolState::Transform,
            },
            // EasyMarkEditorPlugin,
            ReqwestPlugin,
        ))
        .add_state::<AppState>()
        .add_state::<ToolState>()
        .insert_resource(Tool::Labeller)
        .add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::Canvas3d),
        )
        .add_collection_to_loading_state::<AppState, FontAssets>(AppState::Loading)
        .add_collection_to_loading_state::<AppState, GlbAssets>(AppState::Loading)
        .add_systems(Startup, Self::setup)
        .add_systems(Update, file_drag_and_drop);
        // Adds the plugins for each state
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn cleanup<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
