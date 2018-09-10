// Initial thought: let's treat this kind of like shader code:
// we just evaluate the function given the parameters

pub fn sin(hertz: f32, time: f32) -> f32 {
    return (hertz * time).sin();
}

pub fn square(hertz: f32, time: f32) -> f32 {
    if (hertz * time).sin().is_sign_positive() {
        return 1.0;
    } else {
        return -1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;
    extern crate float_cmp;
    use oscillator::tests::float_cmp::ApproxEqUlps;

    #[test]
    fn test_sin() {
        sin(1.0, 0.0).approx_eq_ulps(&0.0, 1);
        sin(1.0, PI / 2.0).approx_eq_ulps(&1.0, 1);
        sin(1.0, PI).approx_eq_ulps(&0.0, 1);
        sin(440.0, PI / 220.0).approx_eq_ulps(&1.0, 1);
        sin(1.0, 90.0).approx_eq_ulps(&0.8939966636, 1);
    }

    #[test]
    fn test_square() {
        square(1.0, 0.0).approx_eq_ulps(&1.0, 1);
        square(1.0, 0.9).approx_eq_ulps(&1.0, 1);
        square(1.0, 1.1).approx_eq_ulps(&-1.0, 1);
        square(1.0, 1.9).approx_eq_ulps(&-1.0, 1);
    }
}
