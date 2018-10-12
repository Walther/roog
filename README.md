# roog

[![Build Status](https://travis-ci.org/Walther/roog.svg?branch=master)](https://travis-ci.org/Walther/roog)

/ˈɹoʊ̯ɡ/ - Synthesizer written in Rust

## roog synth design

### wave stage

- saw, sin, square, triangle wave primitives
- volume control for each determines the mix

### adsr stage

- attack, decay, sustain, release
- for shaping the volume during the lifetime of the note

### filter stage

- ??

### something exciting stage

- ??

### output stage ?

- final volume ?

## TODO

- [x] Oscillator
  - [x] Simple sine wave
  - [x] Square wave
  - [x] Triangle wave
  - [x] Sawtooth wave
- [x] ADSR - Attack, Decay, Sustain, Release
- [x] Monophonic synth
- [x] Polyphonic synth
- [ ] CI builds into VST files
- [ ] GUI, with cross-platform support
- [ ] Something exciting

## Random design thoughts

- This should be highly efficient, real-time capable
- This should be runnable on microcontrollers / embedded
- This should be simple to use, components should be easy to compose
