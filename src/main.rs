mod oscillator;
use oscillator::sin;

fn main() {
    println!("Hello, roog!");

    let hertz = 1.0;
    let mut time = 0.0;
    let mut value;
    println!("Oscillator values:");
    for _ in 0..100 {
        value = sin(hertz, time);
        println!("{},{}", time, value);
        time += 0.01;
    }
    println!("")
}
