mod oscillator;
use oscillator::sin;
use std::f32::consts::PI;

fn main() {
    println!("Hello, roog!");

    let hertz = PI / 2.0;
    let mut time = 0.0;
    let mut value;
    println!("Oscillator values:");
    for _ in 0..10 {
        value = sin(hertz, time);
        print!("{} ", value);
        time += 0.1;
    }
    println!("")
}
