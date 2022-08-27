use rand::prelude::*;


pub fn random_number(min: f32, max: f32) -> f32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(min..max)
}
