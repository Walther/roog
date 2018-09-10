pub fn oscillator(hertz: f32, time: f32) -> f32 {
    // Initial thought: let's treat this kind of like shader code:
    // we just evaluate the function given the parameters
    return (hertz * time).sin();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;
    extern crate float_cmp;
    use oscillator::tests::float_cmp::ApproxEqUlps;

    #[test]
    fn test_oscillator_sin() {
        oscillator(1.0, 0.0).approx_eq_ulps(&0.0, 1);
        oscillator(1.0, PI / 2.0).approx_eq_ulps(&1.0, 1);
        oscillator(1.0, PI).approx_eq_ulps(&0.0, 1);
        oscillator(440.0, PI / 220.0).approx_eq_ulps(&1.0, 1);
        oscillator(1.0, 90.0).approx_eq_ulps(&0.8939966636, 1);
    }
}
