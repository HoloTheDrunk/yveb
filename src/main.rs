use bevy::math::Vec2;
use bevy::prelude::*;
use core::*;

struct Tail {
    joints: Vec<Vec2>,
}

struct Active;

impl Tail {
    pub fn new() -> Tail {
        Tail { joints: Vec::new() }
    }

    pub fn init_joints(&mut self, amount: i32) {
        for i in 0..amount {
            self.joints.push(Vec2::new(i as f32, i as f32));
        }
    }
}

fn add_tails(mut commands: Commands) {
    for _ in 0..10 {
        commands.spawn().insert(Tail::new().init_joints(10));
    }
}

fn tails_follow_cursor(mut query: Query<&mut Tail, With<Active>>) {
    for mut tail in query.iter_mut() {
        for joint_index in 1..tail.joints.len() {
            tail.joints[joint_index] =
                Vec2::lerp(tail.joints[joint_index],
                           tail.joints[joint_index - 1],
                           0.1);
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_tails.system())
        .add_system(tails_follow_cursor.system())
        .run();
}
