# roog

/ˈɹoʊ̯ɡ/ - Synthesizer written in Rust

## TODO

- [ ] Oscillator
  - [x] Simple sine wave
  - [x] Square wave
  - [ ] Triangle wave
  - [ ] Sawtooth wave
- [ ] ADSR - Attack, Decay, Sustain, Release
- [ ] Synth
  - [ ] Monophonic:
  - [ ] Using 1 oscillator
  - [ ] Using 2 oscillators with a blend knob
  - [ ] Using an ADSR for note events
  - [ ] Polyphonic
    - [ ] Shared continuous oscillators (think: oscillator call for each note doesn't always start from 0)
    - [ ] ADSR triggered for each note event separately (think: chord and melody played simultaneously)

## Random design thoughts

- This should be highly efficient, real-time capable
- This should be runnable on microcontrollers / embedded
- This should be simple to use, components should be easy to compose