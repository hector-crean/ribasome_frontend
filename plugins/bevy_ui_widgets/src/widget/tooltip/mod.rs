use bevy::{
    ecs::query::WorldQuery,
    prelude::*,
    ui::{ContentSize, FocusPolicy},
};

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_tooltip_position);
    }
}

/// Marker component present on all tooltips.
#[derive(Component, Default, Debug)]
pub struct Tooltip;

/// Describes the positioning of the tooltip.
///
/// TODO: Add uinode-relative positioning (using Entity + Node + ? )
#[derive(Component, Debug)]
pub enum TooltipAnchor {
    // FollowCursor,
    AlignedToMesh(Entity),
    // Absolute(Vec2),
    // Rect(UiRect),
    Manual,
}

impl Default for TooltipAnchor {
    fn default() -> Self {
        Self::Manual
    }
}

/// Describes the alignment of the tooltip relative to its position.
/// This will be ignored when using a Rect position.
#[derive(Component, Debug)]
pub enum TooltipAlign {
    Bottom,
    Left,
    Right,
    Top,
}

impl Default for TooltipAlign {
    fn default() -> Self {
        Self::Top
    }
}

/// The tooltip's text.
///
/// When present, the text will automatically be copied to the tooltip's child text node.
#[derive(Component)]
pub struct TooltipText(pub Text);

#[derive(Component)]
pub struct TooltipUiNodes {
    /// Root node created via NodeBundle
    pub root: Entity,
    /// Text node created via TextBundle
    pub text: Entity,
}

/// Marker component used to identify the text node.
/// Also contains a reference to the tooltip's root entity.
#[derive(Component)]
pub struct TooltipTextUiNode(pub Entity);

fn update_tooltip_position(
    mut mesh_query: Query<
        // components
        &Transform,
        // filters
        (With<Handle<Mesh>>),
    >,
    mut tooltip_query: Query<TooltipQuery>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
) {
    for (camera, camera_transform) in camera_query.iter() {
        for TooltipQueryItem {
            anchor, mut style, ..
        } in tooltip_query.iter_mut()
        {
            match anchor {
                TooltipAnchor::AlignedToMesh(mesh_entity) => {
                    let mesh_transform = match mesh_query.get_mut(*mesh_entity) {
                        Ok(mesh_transform) => mesh_transform,
                        Err(err) => {
                            info!("could not find mesh");
                            continue;
                        }
                    };

                    match camera.world_to_viewport(&camera_transform, mesh_transform.translation) {
                        Some(coords) => {
                            style.left = Val::Px(coords.x);
                            style.top = Val::Px(coords.y);
                        }
                        None => {
                            // A hack to hide the text when the cube is behind the camera
                            style.bottom = Val::Px(-1000.0);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn calculate_node_point(align: &TooltipAlign, size: &Vec2, position: &Vec2) -> Vec2 {
    match align {
        TooltipAlign::Bottom => Vec2::new(position.x, position.y - (size.y / 2.0)),
        TooltipAlign::Left => Vec2::new(position.x - (size.x / 2.0), position.y),
        TooltipAlign::Right => Vec2::new(position.x + (size.x / 2.0), position.y),
        TooltipAlign::Top => Vec2::new(position.x, position.y + (size.y / 2.0)),
    }
}

fn calculate_tooltip_rect(
    align: &TooltipAlign,
    tooltip_size: &Vec2,
    point: &Vec2,
    offset: f32,
) -> UiRect {
    match align {
        TooltipAlign::Bottom => UiRect {
            bottom: Val::Px(point.y - tooltip_size.y - offset),
            left: Val::Px(point.x - (tooltip_size.x / 2.0)),
            ..default()
        },
        TooltipAlign::Left => UiRect {
            left: Val::Px(point.x - tooltip_size.x - offset),
            bottom: Val::Px(point.y - (tooltip_size.y / 2.0)),
            ..default()
        },
        TooltipAlign::Right => UiRect {
            left: Val::Px(point.x + offset),
            bottom: Val::Px(point.y - (tooltip_size.y / 2.0)),
            ..default()
        },
        TooltipAlign::Top => UiRect {
            bottom: Val::Px(point.y + offset),
            left: Val::Px(point.x - (tooltip_size.x / 2.0)),
            ..default()
        },
    }
}

fn update_text(
    tooltips_q: Query<(&TooltipUiNodes, &TooltipText), Changed<TooltipText>>,
    mut text_node_q: Query<&mut Text, With<TooltipTextUiNode>>,
) {
    for (tooltip_nodes, tooltip_text) in tooltips_q.iter() {
        if let Ok(mut text) = text_node_q.get_mut(tooltip_nodes.text) {
            *text = tooltip_text.0.clone();
        }
    }
}

/// A tooltip is a text container with special positioning rules that displays on top of other UI elements.
/// The recommended way to use tooltips for now is to spawn them "standalone" and not as a child of another UI node.
#[derive(Bundle, Debug)]
pub struct TooltipBundle {
    pub tooltip: Tooltip,
    pub anchor: TooltipAnchor,
    pub align: TooltipAlign,
    /// Describes the size of the node
    pub node: Node,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// Describes the image of the node
    pub image: UiImage,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    pub transform: Transform,
    /// The global transform of the node
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl Default for TooltipBundle {
    fn default() -> Self {
        Self {
            tooltip: default(),
            anchor: TooltipAnchor::default(),
            align: TooltipAlign::default(),
            node: default(),
            style: Self::default_style(),
            image: default(),
            focus_policy: default(),
            transform: default(),
            global_transform: default(),
            visibility: default(),
            computed_visibility: default(),
        }
    }
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct TooltipQuery {
    entity: Entity,
    pub tooltip: &'static Tooltip,
    pub anchor: &'static TooltipAnchor,
    pub align: &'static TooltipAlign,
    /// Describes the size of the node
    pub node: &'static Node,
    /// Describes the style including flexbox settings
    pub style: &'static mut Style,
    /// Describes the image of the node
    pub image: &'static UiImage,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: &'static FocusPolicy,
    /// The transform of the node
    pub transform: &'static Transform,
    /// The global transform of the node
    pub global_transform: &'static GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: &'static Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: &'static ComputedVisibility,
}

impl TooltipBundle {
    pub fn default_style() -> Style {
        Style {
            position_type: PositionType::Absolute,
            border: UiRect::all(Val::Px(2.0)),
            align_items: AlignItems::Center,
            ..default()
        }
    }
}
