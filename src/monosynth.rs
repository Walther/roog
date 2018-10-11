use oscillator::*;

pub struct WaveStage {
  saw_intensity: f64,
  sin_intensity: f64,
  square_intensity: f64,
  triangle_intensity: f64,
}

impl WaveStage {
  pub fn get_sample(&self, hertz: f64, time: f64) -> f64 {
    let oscillator1 = saw(hertz, time, self.saw_intensity);
    let oscillator2 = sin(hertz, time, self.sin_intensity);
    let oscillator3 = square(hertz, time, self.square_intensity);
    let oscillator4 = triangle(hertz, time, self.triangle_intensity);

    return oscillator1 + oscillator2 + oscillator3 + oscillator4;
  }
}

pub struct MonoSynth {
  wave_stage: WaveStage,
}

impl Default for MonoSynth {
  fn default() -> MonoSynth {
    MonoSynth {
      wave_stage: WaveStage {
        saw_intensity: 0.0,
        sin_intensity: 0.2,
        square_intensity: 0.2,
        triangle_intensity: 0.2,
      },
    }
  }
}

impl MonoSynth {
  pub fn new() -> MonoSynth {
    MonoSynth::default()
  }

  pub fn get_sample(&self, hertz: f64, time: f64) -> f64 {
    return self.wave_stage.get_sample(hertz, time);
  }

  pub fn get_parameter(&self, index: i32) -> f64 {
    match index {
      0 => self.wave_stage.saw_intensity,
      1 => self.wave_stage.sin_intensity,
      2 => self.wave_stage.square_intensity,
      3 => self.wave_stage.triangle_intensity,
      _ => 0.0,
    }
  }

  pub fn set_parameter(&mut self, index: i32, val: f64) {
    match index {
      0 => self.wave_stage.saw_intensity = val,
      1 => self.wave_stage.sin_intensity = val,
      2 => self.wave_stage.square_intensity = val,
      3 => self.wave_stage.triangle_intensity = val,
      _ => (),
    }
  }
}
