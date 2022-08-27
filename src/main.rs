use bevy::prelude::*;

fn main() {
  App::new().add_plugin(psim::Simulator).run();
}
