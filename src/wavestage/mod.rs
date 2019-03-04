use crate::oscillator::*;

pub struct WaveStage {
  pub saw_intensity: f64,
  pub sin_intensity: f64,
  pub square_intensity: f64,
  pub triangle_intensity: f64,
}

impl Default for WaveStage {
  fn default() -> WaveStage {
    WaveStage {
      saw_intensity: 0.0,
      sin_intensity: 0.2,
      square_intensity: 0.2,
      triangle_intensity: 0.2,
    }
  }
}

impl WaveStage {
  pub fn new() -> WaveStage {
    WaveStage::default()
  }

  pub fn get_sample(&self, hertz: f64, time: f64) -> f64 {
    let oscillator1 = saw(hertz, time, self.saw_intensity);
    let oscillator2 = sin(hertz, time, self.sin_intensity);
    let oscillator3 = square(hertz, time, self.square_intensity);
    let oscillator4 = triangle(hertz, time, self.triangle_intensity);

    return oscillator1 + oscillator2 + oscillator3 + oscillator4;
  }
}
