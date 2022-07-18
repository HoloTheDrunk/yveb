use bevy::prelude::*;
// serde::{ser, de} for hand (de)serialization
use serde::{Deserialize, Serialize};
use std::fs;

static DEFAULT_SETTINGS_PATH: &str = "assets/config/config.cfg";

#[allow(unused)]
pub struct ConfigPathResource {
    pub path: &'static str,
}

impl FromWorld for ConfigPathResource {
    fn from_world(_world: &mut World) -> Self {
        ConfigPathResource {
            path: DEFAULT_SETTINGS_PATH,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ArrowChannelSettings {
    pub key: KeyCode,

    /// Given as % of the distance between spawn and target per second
    pub scroll_speed: u8,

    /// Given as positive or negative % of the screen width and height
    pub spawn_position: Vec2,
    pub target_position: Vec2,

    /// Given in degrees
    pub local_rotation: f32,
    pub world_rotation: f32,
}

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct GameSettingsResource {
    pub judgements: Vec<usize>,
    pub channels: Vec<ArrowChannelSettings>,
}

/* impl Serialize for GameSettingsResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = 1
            + self.keys.len()
            + self.spawn_positions.len()
            + self.target_positions.len()
            + self.rotations.len();
        let mut s = serializer.serialize_struct("GameSettingsResource", len)?;

        s.serialize_field("scroll_speed", &self.scroll_speed)?;
        s.serialize_field("keys", &self.keys)?;
        s.serialize_field("scroll_speed", &self.scroll_speed)?;
        s.serialize_field("scroll_speed", &self.scroll_speed)?;

        s.end()
    }
} */

impl FromWorld for GameSettingsResource {
    fn from_world(world: &mut World) -> Self {
        let path = match world.get_resource::<ConfigPathResource>() {
            None => DEFAULT_SETTINGS_PATH,
            Some(config_path) => &*config_path.path,
        };

        let config_file = fs::read_to_string(&*path)
            .unwrap_or_else(|_| format!("Couldn't locate settings file at  `{}'", path));

        let settings: GameSettingsResource =
            serde_yaml::from_str(config_file.as_str()).expect("Malformed config file");

        settings
    }
}

fn save_settings(config_path: Res<ConfigPathResource>, settings: Res<GameSettingsResource>) {
    fs::write(
        config_path.path,
        serde_yaml::to_string(&*settings).expect("Internal config error"),
    )
    .expect("Couldn't write config");
}

#[allow(dead_code)]
fn generate_default_settings() {
    let settings = GameSettingsResource {
        judgements: vec![16, 40, 73, 104, 127],
        channels: vec![ArrowChannelSettings {
            key: KeyCode::D,
            scroll_speed: 100,
            spawn_position: Vec2::new(0., 0.),
            target_position: Vec2::new(0., 0.),
            local_rotation: 0.,
            world_rotation: 0.,
        }],
    };

    fs::write(
        "config-default",
        serde_yaml::to_string(&settings).expect("INTERNAL CONFIG ERROR"),
    )
    .expect("Couldn't write config");
}

pub struct GameSettingsPlugin;
impl Plugin for GameSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettingsResource>()
            .add_startup_system(save_settings);
    }
}
