use bevy::{input::system::exit_on_esc_system, prelude::*};

mod arrows;
use arrows::ArrowsPlugin;

mod settings;
use settings::{ConfigPathResource, GameSettingsPlugin};

mod song;
use song::{SongConfig, SongPathResource, SongPlugin};

#[allow(unused)]
fn window_to_world(position: Vec2, window: &Window, camera: &Transform) -> Vec3 {
    // Center in screen space
    let norm = Vec3::new(
        position.x - window.width() / 2.,
        position.y - window.height() / 2.,
        0.,
    );

    // Apply camera transform
    *camera * norm
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}

fn print_time(time: Res<Time>) {
    println!("Delta: {:?}", time.delta());
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "YVEB".to_string(),
            ..Default::default()
        })
        .insert_resource(ConfigPathResource {
            path: "assets/config/config.cfg",
        })
        .insert_resource(SongPathResource {
            path: "assets/songs/song_test.csv",
        })
        .add_startup_system(setup)
        .add_system(print_time)
        .add_system(exit_on_esc_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(GameSettingsPlugin)
        .add_plugin(ArrowsPlugin)
        .add_plugin(SongPlugin)
        .run();
}
