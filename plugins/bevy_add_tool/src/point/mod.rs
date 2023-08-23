use bevy::{a11y::accesskit::Point, prelude::*};
use bevy_mod_picking::{
    backend::HitData,
    events::{Click, Down, Pointer},
};
use bevy_mod_reqwest::*;
use bevy_ui_widgets::widget::tooltip::*;

#[derive(Resource, Default)]
pub struct PointTool;

impl PointTool {
    pub fn emit_point_tool_event(
        mut pointerdowns: EventReader<Pointer<Down>>,
        mut point_tool_evt_wtr: EventWriter<PointToolCommand>,
    ) {
        for pointerdown in pointerdowns.iter() {
            let HitData { position, .. } = pointerdown.event.hit;

            match position {
                Some(position) => point_tool_evt_wtr.send(PointToolCommand::Create { position }),
                None => {}
            }
        }
    }
    pub fn consume_point_tool_event(
        mut point_tool_evts: EventReader<PointToolCommand>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut asset_server: ResMut<AssetServer>,
    ) {
        for ev in point_tool_evts.iter() {
            match ev {
                PointToolCommand::Create { position } => {
                    let mesh_entity = commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.05 })),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_translation(*position),
                            ..default()
                        })
                        .id();

                    let tooltip_content = commands
                        .spawn((
                            TextBundle {
                                text: Text::from_section(
                                    "This is example text",
                                    TextStyle {
                                        font: asset_server
                                            .load("fonts/iosevka-extendedheavyoblique.ttf"),
                                        font_size: 10.,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                style: Style {
                                    width: Val::Px(120.),
                                    height: Val::Px(120.),
                                    border: UiRect::all(Val::Px(2.)),
                                    display: Display::Flex,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,

                                    ..default()
                                },
                                background_color: Color::rgba(
                                    108. / 180.,
                                    130. / 180.,
                                    142. / 180.,
                                    0.1,
                                )
                                .into(),
                                ..default()
                            },
                            Label,
                        ))
                        .id();

                    commands
                        .spawn(
                            (TooltipBundle {
                                anchor: TooltipAnchor::AlignedToMesh(mesh_entity),
                                ..default()
                            }),
                        )
                        .add_child(tooltip_content);

                    // if let Ok(url) = "https://www.boredapi.com/api/activity".try_into() {
                    //     let req = reqwest::Request::new(reqwest::Method::GET, url);
                    //     let req = ReqwestRequest::new(req);
                    //     commands.spawn(req);
                    // }
                }
            }
        }
    }
}

#[derive(Event)]
pub enum PointToolCommand {
    Create { position: Vec3 },
}
