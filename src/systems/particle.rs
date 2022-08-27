use bevy::prelude::*;

use crate::components::{Particle, Velocity};
use crate::resources::ParticleInteractions;
use crate::utils::random_number;


pub fn spawn_particles(mut commands: Commands) {
  let particle_count = 250;

  for _ in 0..particle_count {
    commands.spawn()
      .insert(Particle::Red)
      .insert(Velocity { x: 0f32, y: 0f32 })
      .insert_bundle(SpriteBundle {
        sprite: Sprite {
          color: Color::rgb(1.0, 0.0, 0.0),
          custom_size: Some(Vec2::new(5.0, 5.0)),
          ..default()
        },
        transform: Transform::from_xyz(
          random_number(-400f32, 400f32),
          random_number(-300f32, 300f32),
          0f32,
        ),
        ..default()
      });
  }

  for _ in 0..particle_count {
    commands.spawn()
      .insert(Particle::Green)
      .insert(Velocity { x: 0f32, y: 0f32 })
      .insert_bundle(SpriteBundle {
        sprite: Sprite {
          color: Color::rgb(0.0, 1.0, 0.0),
          custom_size: Some(Vec2::new(5.0, 5.0)),
          ..default()
        },
        transform: Transform::from_xyz(
          random_number(-400f32, 400f32),
          random_number(-300f32, 300f32),
          0f32,
        ),
        ..default()
      });
  }

  for _ in 0..particle_count {
    commands.spawn()
      .insert(Particle::Blue)
      .insert(Velocity { x: 0f32, y: 0f32 })
      .insert_bundle(SpriteBundle {
        sprite: Sprite {
          color: Color::rgb(0.0, 0.0, 1.0),
          custom_size: Some(Vec2::new(5.0, 5.0)),
          ..default()
        },
        transform: Transform::from_xyz(
          random_number(-400f32, 400f32),
          random_number(-300f32, 300f32),
          0f32,
        ),
        ..default()
      });
  }

  for _ in 0..particle_count {
    commands.spawn()
      .insert(Particle::White)
      .insert(Velocity { x: 0f32, y: 0f32 })
      .insert_bundle(SpriteBundle {
        sprite: Sprite {
          color: Color::rgb(1.0, 1.0, 1.0),
          custom_size: Some(Vec2::new(5.0, 5.0)),
          ..default()
        },
        transform: Transform::from_xyz(
          random_number(-400f32, 400f32),
          random_number(-300f32, 300f32),
          0f32,
        ),
        ..default()
      });
  }
}

pub fn compute_particles_velocity(
  interactions: Res<ParticleInteractions>,
  time: Res<Time>,
  mut query: Query<(&Particle, &Transform, &mut Velocity)>
) {
  let mut iter = query.iter_combinations_mut();

  while let Some([source, other]) = iter.fetch_next() {
    let (
      source_particle,
      source_transform,
      mut source_velocity,
    ) = source;

    let (
      other_particle,
      other_transform,
      _,
    ) = other;

    let interaction = interactions.get_interaction(source_particle, other_particle);

    let dx = source_transform.translation.x - other_transform.translation.x;
    let dy = source_transform.translation.y - other_transform.translation.y;
    let d = (dx * dx + dy * dy).sqrt();

    if d > 0f32 && d < 80f32 {
      let force = interaction * 1f32 / d;
      source_velocity.x += force * dx * time.delta_seconds();
      source_velocity.y += force * dy * time.delta_seconds();
    }
  }
}

pub fn move_particles(mut query: Query<(&mut Transform, &mut Velocity)>) {
  for (mut transform, mut velocity) in query.iter_mut() {
    transform.translation.x += velocity.x;
    transform.translation.y += velocity.y;

    if transform.translation.x < -400.0 || transform.translation.x > 400.0 {
      velocity.x *= -1.0;
    }

    if transform.translation.y < -300.0 || transform.translation.y > 300.0 {
      velocity.y *= -1.0;
    }
  }
}
