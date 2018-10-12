use adsrstage::*;
use noteutils::*;
use wavestage::*;

pub struct MonoSynth {
  sample_rate: f64,
  wave_stage: WaveStage,
  adsr_stage: ADSRStage,
  note: Note,
}

impl Default for MonoSynth {
  fn default() -> MonoSynth {
    MonoSynth {
      sample_rate: 44100.0,
      wave_stage: WaveStage::new(),
      adsr_stage: ADSRStage::new(),
      note: Note::default(),
    }
  }
}

impl MonoSynth {
  pub fn new() -> MonoSynth {
    MonoSynth::default()
  }

  pub fn note_on(&mut self, note: f64) {
    self.note.value = note;
    self.note.duration = 0.0;
    self.note.release_time = 0.0;
    self.note.state = NoteState::Attack;
  }

  pub fn note_off(&mut self, note: f64) {
    if self.note.value == note {
      self.note.release_time = self.note.duration;
      self.note.state = NoteState::Release;
    }
  }

  pub fn set_sample_rate(&mut self, rate: f64) {
    self.sample_rate = rate;
  }

  fn time_per_sample(&self) -> f64 {
    1.0 / self.sample_rate
  }

  pub fn get_sample(&mut self, time: f64) -> f64 {
    let mut mix = 0.0f64;

    if self.note.state != NoteState::Off {
      // Wave stage
      let per_sample = self.time_per_sample();
      let hertz = self.note.value;
      let value = self.wave_stage.get_sample(hertz, time);

      // ADSR stage
      mix = self.adsr_stage.adsr(&mut self.note, value);

      // Increment duration
      self.note.duration += per_sample;
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
