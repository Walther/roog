use oscillator::{saw, sin, square, triangle};

pub struct MonoSynth {
  saw: f64,
  sin: f64,
  square: f64,
  triangle: f64,
}

impl Default for MonoSynth {
  fn default() -> MonoSynth {
    MonoSynth {
      saw: 0.0,
      sin: 0.2,
      square: 0.2,
      triangle: 0.2,
    }
  }
}

impl MonoSynth {
  pub fn new() -> MonoSynth {
    MonoSynth::default()
  }

  pub fn get_parameter(&self, index: i32) -> f64 {
    match index {
      0 => self.saw,
      1 => self.sin,
      2 => self.square,
      3 => self.triangle,
      _ => 0.0,
    }
  }

  pub fn set_parameter(&mut self, index: i32, val: f64) {
    match index {
      0 => self.saw = val,
      1 => self.sin = val,
      2 => self.square = val,
      3 => self.triangle = val,
      _ => (),
    }
  }

  pub fn get_sample(&self, hertz: f64, time: f64) -> f64 {
    let oscillator1 = self.saw * saw(hertz, time);
    let oscillator2 = self.sin * sin(hertz, time);
    let oscillator3 = self.square * square(hertz, time);
    let oscillator4 = self.triangle * triangle(hertz, time);

    let mix = oscillator1 + oscillator2 + oscillator3 + oscillator4;

    return mix;
  }
}
