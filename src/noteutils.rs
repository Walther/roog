#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NoteState {
  Attack,
  Decay,
  Sustain,
  Release,
  Off,
}

#[derive(Copy, Clone, Debug)]
pub struct Note {
  pub state: NoteState,
  pub duration: f64,
  pub release_time: f64,
  pub value: f64,
}

impl Default for Note {
  fn default() -> Note {
    Note {
      state: NoteState::Off,
      duration: 0.0,
      release_time: 0.0,
      value: 0.0,
    }
  }
}

pub struct Keyboard {
  pub notes: [Note; 128],
}

impl Default for Keyboard {
  fn default() -> Keyboard {
    Keyboard {
      notes: [Note::default(); 128],
    }
  }
}

impl Keyboard {
  pub fn new() -> Keyboard {
    Keyboard::default()
  }
}
