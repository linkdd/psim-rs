use bevy::prelude::*;


pub fn initialize_camera(mut commands: Commands) {
  commands.spawn_bundle(Camera2dBundle::default());
}
