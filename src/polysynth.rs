use adsrstage::*;
use noteutils::*;
use wavestage::*;

pub struct PolySynth {
  sample_rate: f64,
  wave_stage: WaveStage,
  adsr_stage: ADSRStage,
  keyboard: Keyboard,
}

impl Default for PolySynth {
  fn default() -> PolySynth {
    PolySynth {
      sample_rate: 44100.0,
      wave_stage: WaveStage::new(),
      adsr_stage: ADSRStage::new(),
      keyboard: Keyboard::new(),
    }
  }
}

impl PolySynth {
  pub fn new() -> PolySynth {
    PolySynth::default()
  }

  pub fn note_on(&mut self, hertz: f64) {
    // Try to find a free slot in the 128-voice synth
    // Prefer a note with the same hertz already, if e.g. in Release phase
    if let Some(note) = self
      .keyboard
      .notes
      .iter_mut()
      .find(|note| note.value == hertz || note.state == NoteState::Off)
    {
      // if we find one, set it on!
      note.value = hertz;
      note.duration = 0.0;
      note.release_time = 0.0;
      note.state = NoteState::Attack;
    }
  }

  pub fn note_off(&mut self, hertz: f64) {
    // Try to find the hertz to stop in the 128-voice synth
    if let Some(note) = self
      .keyboard
      .notes
      .iter_mut()
      .find(|note| note.value == hertz)
    {
      note.release_time = note.duration;
      note.state = NoteState::Release;
    }
  }

  pub fn set_sample_rate(&mut self, rate: f64) {
    self.sample_rate = rate;
  }

  fn time_per_sample(&self) -> f64 {
    1.0 / self.sample_rate
  }

  pub fn get_sample(&mut self, time: f64) -> f64 {
    let per_sample = self.time_per_sample();
    let mut mix = 0.0f64;

    for mut note in self.keyboard.notes.iter_mut() {
      // Wave stage
      let hertz = note.value;
      let value = self.wave_stage.get_sample(hertz, time);

      // ADSR stage
      mix += self.adsr_stage.adsr(&mut note, value);

      // Increment duration
      note.duration += per_sample;
    }

    return mix;
  }

  pub fn get_parameter(&self, index: i32) -> f64 {
    match index {
      0 => self.wave_stage.saw_intensity,
      1 => self.wave_stage.sin_intensity,
      2 => self.wave_stage.square_intensity,
      3 => self.wave_stage.triangle_intensity,

      4 => self.adsr_stage.attack,
      5 => self.adsr_stage.decay,
      6 => self.adsr_stage.sustain,
      7 => self.adsr_stage.release,
      _ => 0.0,
    }
  }

  pub fn set_parameter(&mut self, index: i32, val: f64) {
    match index {
      0 => self.wave_stage.saw_intensity = val,
      1 => self.wave_stage.sin_intensity = val,
      2 => self.wave_stage.square_intensity = val,
      3 => self.wave_stage.triangle_intensity = val,

      4 => self.adsr_stage.attack = val,
      5 => self.adsr_stage.decay = val,
      6 => self.adsr_stage.sustain = val,
      7 => self.adsr_stage.release = val,
      _ => (),
    }
  }
}
