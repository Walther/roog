extern crate cpal;

mod oscillator;
use oscillator::sin;

fn main() {
    println!("Hello, roog!");

    // CPAL initialization
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let sample_rate = format.sample_rate.0 as f32;
    let mut sample_clock = 0f32;

    // Oscillator initialization
    let hertz = 440.0;
    let mut time = 0.0; // Should be seconds

    // Code adapted from CAPL example. TODO: understand, write own
    let mut oscillator = || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        time = sample_clock / sample_rate; // Should be seconds
        return sin(hertz, time);
    };

    // CPAL example main loop. TODO: understand, write own
    event_loop.run(move |_, data| {
        match data {
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = oscillator();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            _ => (),
        }
    });
}
