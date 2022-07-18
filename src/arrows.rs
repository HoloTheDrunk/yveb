use crate::settings::GameSettingsResource;
use crate::song::SongConfig;
use bevy::prelude::*;
use std::f32::consts::TAU;

/// Image handles used to render notes and targets.
#[allow(unused)]
struct ArrowMaterialResource {
    fill_texture: Handle<Image>,
    border_texture: Handle<Image>,
}

impl FromWorld for ArrowMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let fill_handle = asset_server.load("images/arrow_fill.png");
        let border_handle = asset_server.load("images/arrow_border.png");

        ArrowMaterialResource {
            fill_texture: fill_handle,
            border_texture: border_handle,
        }
    }
}

/// Struct representing the necessary information about a note.
#[derive(Component, Copy, Clone, Debug)]
pub struct Arrow {
    /// Index of the note channel this note belongs to.
    pub channel: usize,
    /// Timing in milliseconds of when the note should be hit.
    pub timing: usize,
    /// Either [Short] or [Long].
    pub kind: ArrowKind,
}

#[derive(Copy, Clone, Debug)]
pub enum ArrowKind {
    Short,
    Long { duration: usize },
}

#[derive(Component, Debug)]
pub struct TargetArrow;

fn key_pressed(settings: Res<GameSettingsResource>, input: Res<Input<KeyCode>>) {
    for key in settings.channels.iter().map(|chan| chan.key) {
        if input.just_pressed(key) {
            println!("Pressed {:?}", key);
        }
    }
}

fn setup_target_arrows(
    mut commands: Commands,
    settings: Res<GameSettingsResource>,
    materials: Res<ArrowMaterialResource>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().expect("Window not initialized");
    let (width, height) = (window.width(), window.height());

    for channel in settings.channels.iter() {
        let transform = Transform::from_translation(Vec3::new(
            width * channel.target_position.x / 100. / 2.,
            height * channel.target_position.y / 100. / 2.,
            1.,
        ))
        .with_rotation(Quat::from_rotation_z(TAU * channel.local_rotation / 360.));

        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(140., 140.)),
                    ..Default::default()
                },
                texture: materials.border_texture.clone(),
                transform,
                ..Default::default()
            })
            .insert(TargetArrow);
    }
}

fn spawn_arrows(
    mut commands: Commands,
    materials: Res<ArrowMaterialResource>,
    settings: Res<GameSettingsResource>,
    song: ResMut<SongConfig>,
    windows: Res<Windows>,
    time: Res<Time>,
) {
    let window = windows.get_primary().expect("Window not initialized");
    let (width, _height) = (window.width(), window.height());

    for note in song.notes.iter() {
        if note.channel > settings.channels.len() { continue; }
        let channel = &settings.channels[note.channel];

        let current_time: usize = time.time_since_startup().as_millis().try_into().unwrap();

        // TODO: Figure out the math for this
        if current_time < note.timing || current_time - note.timing > 50 { break; }
        // if note.timing
        //     > (time.time_since_startup().as_millis()
        //         - (channel.scroll_speed as f32 * 100.
        //             / (channel.target_position.distance(channel.spawn_position)))
        //             as u128)
        //         .try_into()
        //         .unwrap()
        // {
        //     break;
        // }

        let transform = Transform {
            translation: Vec3::new(
                channel.spawn_position.x,
                channel.spawn_position.y,
                1.,
            ),
            ..Default::default()
        };

        // TODO: Spawn arrow with correct color
        commands.spawn().insert(*note).insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0., 0.),
                custom_size: Some(Vec2::new(140., 140.)),
                ..Default::default()
            },
            texture: materials.fill_texture.clone(),
            // texture: color_materials
            //     .get(materials.fill_texture.clone())
            //     .unwrap()
            //     .clone()
            //     .texture
            //     .unwrap(),
            transform,
            ..Default::default()
        });
    }
}

fn move_arrows(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Arrow)>,
    settings: Res<GameSettingsResource>,
) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x +=
            time.delta_seconds() * settings.channels[arrow.channel].scroll_speed as f32;
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArrowMaterialResource>()
            .add_startup_system(setup_target_arrows)
            .add_system(spawn_arrows)
            .add_system(move_arrows)
            .add_system(key_pressed);
    }
}
