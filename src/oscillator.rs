use std::f64::consts::PI;

pub fn sin(hertz: f64, time: f64, intensity: f64) -> f64 {
    return (2.0 * PI * hertz * time).sin() * intensity;
}

pub fn square(hertz: f64, time: f64, intensity: f64) -> f64 {
    if (2.0 * PI * hertz * time).sin().is_sign_positive() {
        return 1.0 * intensity;
    } else {
        return -1.0 * intensity;
    }
}

pub fn saw(hertz: f64, time: f64, intensity: f64) -> f64 {
    let x = hertz * time;
    return 2.0 * (x - (0.5 + x).floor()) * intensity;
}

pub fn triangle(hertz: f64, time: f64, intensity: f64) -> f64 {
    // Absolute value of sawtooth + shifting
    // NOTE: currently produces a V wave (x, y)
    // (0, 1) (0.5, -1) (1, 1)
    // Is this okay? Should it be shifted to resemble sin?
    return ((2.0 * (saw(hertz, time, 1.0).abs() - 1.0).abs()) - 1.0) * intensity;
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate float_cmp;
    use crate::oscillator::tests::float_cmp::ApproxEqUlps;

    #[test]
    fn test_sin() {
        // Shape tests
        assert!(sin(1.0, 0.0, 1.0).approx_eq_ulps(&0.0, 1));
        assert!(sin(440.0, 1.0 / 1760.0, 1.0).approx_eq_ulps(&1.0, 1));
        // Shape tests, half intensity
        assert!(sin(1.0, 0.0, 0.5).approx_eq_ulps(&0.0, 1));
        assert!(sin(440.0, 1.0 / 1760.0, 0.5).approx_eq_ulps(&0.5, 1));
    }

    #[test]
    fn test_square() {
        // Shape tests
        assert!(square(1.0, 0.0, 1.0).approx_eq_ulps(&1.0, 1));
        assert!(square(1.0, 0.49, 1.0).approx_eq_ulps(&1.0, 1));
        assert!(square(1.0, 0.51, 1.0).approx_eq_ulps(&-1.0, 1));
        assert!(square(1.0, 1.51, 1.0).approx_eq_ulps(&-1.0, 1));

        // Shape tests, half intensity
        assert!(square(1.0, 0.0, 0.5).approx_eq_ulps(&0.5, 1));
        assert!(square(1.0, 0.49, 0.5).approx_eq_ulps(&0.5, 1));
        assert!(square(1.0, 0.51, 0.5).approx_eq_ulps(&-0.5, 1));
        assert!(square(1.0, 1.51, 0.5).approx_eq_ulps(&-0.5, 1));
    }

    #[test]
    fn test_saw() {
        // Shape tests
        assert!(saw(1.0, 0.0, 1.0).approx_eq_ulps(&0.0, 1));
        assert!(saw(1.0, 0.4999999999999999, 1.0).approx_eq_ulps(&1.0, 2));
        assert!(saw(1.0, 0.5, 1.0).approx_eq_ulps(&-1.0, 1));
        assert!(saw(1.0, 1.5, 1.0).approx_eq_ulps(&-1.0, 1));
        assert!(saw(1.0, 0.25, 1.0).approx_eq_ulps(&0.5, 1));

        // Shape tests, half intensity
        assert!(saw(1.0, 0.0, 0.5).approx_eq_ulps(&0.0, 1));
        assert!(saw(1.0, 0.4999999999999999, 0.5).approx_eq_ulps(&0.5, 2));
        assert!(saw(1.0, 0.5, 0.5).approx_eq_ulps(&-0.5, 1));
        assert!(saw(1.0, 1.5, 0.5).approx_eq_ulps(&-0.5, 1));
        assert!(saw(1.0, 0.25, 0.5).approx_eq_ulps(&0.25, 1));
    }

    #[test]
    fn test_triangle() {
        // Shape tests
        assert!(triangle(1.0, 0.0, 1.0).approx_eq_ulps(&1.0, 1));
        assert!(triangle(1.0, 0.5, 1.0).approx_eq_ulps(&-1.0, 1));
        assert!(triangle(1.0, 1.0, 1.0).approx_eq_ulps(&1.0, 1));

        // Shape tests, half intensity
        assert!(triangle(1.0, 0.0, 0.5).approx_eq_ulps(&0.5, 1));
        assert!(triangle(1.0, 0.5, 0.5).approx_eq_ulps(&-0.5, 1));
        assert!(triangle(1.0, 1.0, 0.5).approx_eq_ulps(&0.5, 1));
    }
}
