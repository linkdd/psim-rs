use crate::components::Particle;
use crate::utils::random_number;


pub struct ParticleInteractions {
  pub red_red: f32,
  pub red_green: f32,
  pub red_blue: f32,
  pub red_white: f32,

  pub green_red: f32,
  pub green_green: f32,
  pub green_blue: f32,
  pub green_white: f32,

  pub blue_red: f32,
  pub blue_green: f32,
  pub blue_blue: f32,
  pub blue_white: f32,

  pub white_red: f32,
  pub white_green: f32,
  pub white_blue: f32,
  pub white_white: f32,
}


impl ParticleInteractions {
  pub fn get_interaction(&self, source: &Particle, other: &Particle) -> f32 {
    match (source, other) {
      (Particle::Red, Particle::Red) => self.red_red,
      (Particle::Red, Particle::Green) => self.red_green,
      (Particle::Red, Particle::Blue) => self.red_blue,
      (Particle::Red, Particle::White) => self.red_white,

      (Particle::Green, Particle::Red) => self.green_red,
      (Particle::Green, Particle::Green) => self.green_green,
      (Particle::Green, Particle::Blue) => self.green_blue,
      (Particle::Green, Particle::White) => self.green_white,

      (Particle::Blue, Particle::Red) => self.blue_red,
      (Particle::Blue, Particle::Green) => self.blue_green,
      (Particle::Blue, Particle::Blue) => self.blue_blue,
      (Particle::Blue, Particle::White) => self.blue_white,

      (Particle::White, Particle::Red) => self.white_red,
      (Particle::White, Particle::Green) => self.white_green,
      (Particle::White, Particle::Blue) => self.white_blue,
      (Particle::White, Particle::White) => self.white_white,
    }
  }
}

impl Default for ParticleInteractions {
  fn default() -> Self {
      Self {
        red_red: random_number(-1f32, 1f32),
        red_green: random_number(-1f32, 1f32),
        red_blue: random_number(-1f32, 1f32),
        red_white: random_number(-1f32, 1f32),

        green_red: random_number(-1f32, 1f32),
        green_green: random_number(-1f32, 1f32),
        green_blue: random_number(-1f32, 1f32),
        green_white: random_number(-1f32, 1f32),

        blue_red: random_number(-1f32, 1f32),
        blue_green: random_number(-1f32, 1f32),
        blue_blue: random_number(-1f32, 1f32),
        blue_white: random_number(-1f32, 1f32),

        white_red: random_number(-1f32, 1f32),
        white_green: random_number(-1f32, 1f32),
        white_blue: random_number(-1f32, 1f32),
        white_white: random_number(-1f32, 1f32),
      }
  }
}
