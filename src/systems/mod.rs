mod setup;
mod particle;

pub use self::{
  setup::{
    initialize_camera,
  },
  particle::{
    spawn_particles,
    compute_particles_velocity,
    move_particles,
  },
};
