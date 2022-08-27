use bevy::prelude::*;
use bevy::time::FixedTimestep;

pub(crate) mod utils;
pub(crate) mod resources;
pub(crate) mod components;
pub(crate) mod systems;


pub struct Simulator;

impl Plugin for Simulator {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ClearColor(Color::DARK_GRAY))
      .insert_resource(WindowDescriptor {
        title: "PSim v0.1.0".to_string(),
        width: 800.0,
        height: 600.0,
        resizable: false,
        ..default()
      })
      .add_plugins(DefaultPlugins)
      .insert_resource(resources::ParticleInteractions::default())
      .add_startup_system(systems::initialize_camera)
      .add_startup_system(systems::spawn_particles)
      .add_system_set(
        SystemSet::new()
          .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
          .with_system(systems::compute_particles_velocity)
          .with_system(systems::move_particles)
      );
  }
}
