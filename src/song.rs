use crate::arrows::{Arrow, ArrowKind};
use bevy::prelude::*;
use std::fs;

static DEFAULT_SONG_PATH: &str = "assets/songs/song_test.csv";

pub struct SongPathResource {
    pub path: &'static str,
}

#[derive(Debug)]
pub struct SongConfig {
    pub notes: Vec<Arrow>,
}

impl FromWorld for SongConfig {
    fn from_world(world: &mut World) -> Self {
        let path = match world.get_resource::<SongPathResource>() {
            None => DEFAULT_SONG_PATH,
            Some(config_path) => &*config_path.path,
        };

        let song_file = fs::read_to_string(&*path)
            .unwrap_or_else(|_| format!("Couldn't locate song file at  `{}'", path));

        let mut reader = csv::Reader::from_reader(song_file.as_bytes());

        let mut song = SongConfig { notes: vec![] };

        for note in reader.records() {
            println!("Parsing note {:?}.", note);

            match note {
                Ok(arr) => song.notes.push(match arr[0].chars().next().unwrap() {
                    'S' => Arrow {
                        channel: arr[1].parse::<usize>().unwrap(),
                        timing: arr[2].parse::<usize>().unwrap(),
                        kind: ArrowKind::Short,
                    },
                    'L' => Arrow {
                        channel: arr[1].parse::<usize>().unwrap(),
                        timing: arr[2].parse::<usize>().unwrap(),
                        kind: ArrowKind::Long {
                            duration: arr[3].parse::<usize>().unwrap(),
                        },
                    },
                    _ => panic!("Malformed song file."),
                }),
                _ => eprintln!("Error when parsing note {:?}.", note),
            }
        }

        song
    }
}

pub struct SongPlugin;
impl Plugin for SongPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SongConfig>();
        let song_config: Option<&SongConfig> = app.world.get_resource::<SongConfig>();
        match song_config {
            None => eprintln!("Failed to read song_config"),
            Some(config) => println!("{:?}", config),
        };
    }
}
