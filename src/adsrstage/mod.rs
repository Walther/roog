use noteutils::*;

pub struct ADSRStage {
  pub attack: f64,
  pub decay: f64,
  pub sustain: f64,
  pub release: f64,
}

impl Default for ADSRStage {
  fn default() -> ADSRStage {
    ADSRStage {
      attack: 0.0,
      decay: 0.0,
      sustain: 1.0,
      release: 0.0,
    }
  }
}

impl ADSRStage {
  pub fn new() -> ADSRStage {
    ADSRStage::default()
  }

  pub fn adsr(&self, note: &mut Note, value: f64) -> f64 {
    let mut mix = 0.0f64;
    // ADSR stage
    match note.state {
      // FIXME: Naive implementations: linear, not exponential
      NoteState::Attack => {
        // FIXME: Naive assumption: attack time 0-1 seconds
        let current_attack_ratio = note.duration / self.attack; // ratio gets us 0..1 range regardless of concrete attack value
        if current_attack_ratio < 1.0 {
          mix += value * current_attack_ratio;
        } else {
          mix += value;
          note.state = NoteState::Decay;
        }
      }
      NoteState::Decay => {
        // FIXME: Naive assumption: decay time 0-1 seconds
        let current_decay_ratio = (note.duration - self.attack) / self.decay; // ratio gets us 0..1 range regardless of concrete decay value
        if current_decay_ratio < 1.0 {
          mix += value - value * (1.0 - self.sustain) * current_decay_ratio;
        //mix += value - value * self.sustain * current_decay_ratio; // sustain is the level we are decaying to
        } else {
          mix += value * self.sustain;
          note.state = NoteState::Sustain;
        }
      }
      NoteState::Sustain => {
        mix += value * self.sustain;
      }
      NoteState::Release => {
        // FIXME: Naive assumption: release time 0-1 seconds
        let current_fade_ratio = (note.duration - note.release_time) / self.release;
        if current_fade_ratio < 1.0 {
          mix += value * self.sustain * (1.0 - current_fade_ratio); // ratio gets us 0..1 range regardless of concrete release value
        } else {
          note.state = NoteState::Off;
        }
      }
      NoteState::Off => (),
    }

    return mix;
  }
}
