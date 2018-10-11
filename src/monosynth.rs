use noteutils::*;
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

pub struct ADSRStage {
  attack: f64,
  decay: f64,
  sustain: f64,
  release: f64,
}

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
      wave_stage: WaveStage {
        saw_intensity: 0.0,
        sin_intensity: 0.2,
        square_intensity: 0.2,
        triangle_intensity: 0.2,
      },
      adsr_stage: ADSRStage {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        release: 0.0,
      },
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
    // Wave stage
    let per_sample = self.time_per_sample();
    let hertz = self.note.value;
    let value = self.wave_stage.get_sample(hertz, time);
    let mut mix = 0.0f64;

    // ADSR stage
    match self.note.state {
      // FIXME: Naive implementations: linear, not exponential
      NoteState::Attack => {
        // FIXME: Naive assumption: attack time 0-1 seconds
        let current_attack_ratio = self.note.duration / self.adsr_stage.attack; // ratio gets us 0..1 range regardless of concrete attack value
        if current_attack_ratio < 1.0 {
          mix += value * current_attack_ratio;
        } else {
          mix += value;
          self.note.state = NoteState::Decay;
        }
        self.note.duration += per_sample;
      }
      NoteState::Decay => {
        // FIXME: Naive assumption: decay time 0-1 seconds
        let current_decay_ratio =
          (self.note.duration - self.adsr_stage.attack) / self.adsr_stage.decay; // ratio gets us 0..1 range regardless of concrete decay value
        if current_decay_ratio < 1.0 {
          mix += value - value * (1.0 - self.adsr_stage.sustain) * current_decay_ratio;
        //mix += value - value * self.adsr_stage.sustain * current_decay_ratio; // sustain is the level we are decaying to
        } else {
          mix += value * self.adsr_stage.sustain;
          self.note.state = NoteState::Sustain;
        }
        self.note.duration += per_sample;
      }
      NoteState::Sustain => {
        mix += value * self.adsr_stage.sustain;
        self.note.duration += per_sample;
      }
      NoteState::Release => {
        // FIXME: Naive assumption: release time 0-1 seconds
        let current_fade_ratio =
          (self.note.duration - self.note.release_time) / self.adsr_stage.release;
        if current_fade_ratio < 1.0 {
          mix += value * self.adsr_stage.sustain * (1.0 - current_fade_ratio); // ratio gets us 0..1 range regardless of concrete release value
          self.note.duration += per_sample;
        } else {
          self.note.state = NoteState::Off;
        }
      }
      NoteState::Off => (),
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
