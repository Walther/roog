use std::f64::consts::PI;

// Initial thought: let's treat this kind of like shader code:
// we just evaluate the function given the parameters

pub fn sin(hertz: f64, time: f64) -> f64 {
    return (2.0 * PI * hertz * time).sin();
}

pub fn square(hertz: f64, time: f64) -> f64 {
    if (2.0 * PI * hertz * time).sin().is_sign_positive() {
        return 1.0;
    } else {
        return -1.0;
    }
}

pub fn saw(hertz: f64, time: f64) -> f64 {
    let x = hertz * time;
    return 2.0 * (x - (0.5 + x).floor());
}

pub fn triangle(hertz: f64, time: f64) -> f64 {
    // Absolute value of sawtooth + shifting
    // NOTE: currently produces a V wave (x, y)
    // (0, 1) (0.5, -1) (1, 1)
    // Is this okay? Should it be shifted to resemble sin?
    return (2.0 * (saw(hertz, time).abs() - 1.0).abs()) - 1.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate float_cmp;
    use oscillator::tests::float_cmp::ApproxEqUlps;

    #[test]
    fn test_sin() {
        assert!(sin(1.0, 0.0).approx_eq_ulps(&0.0, 1));
        assert!(sin(440.0, 1.0 / 1760.0).approx_eq_ulps(&1.0, 1));
    }

    #[test]
    fn test_square() {
        assert!(square(1.0, 0.0).approx_eq_ulps(&1.0, 1));
        assert!(square(1.0, 0.49).approx_eq_ulps(&1.0, 1));
        assert!(square(1.0, 0.51).approx_eq_ulps(&-1.0, 1));
        assert!(square(1.0, 1.51).approx_eq_ulps(&-1.0, 1));
    }

    #[test]
    fn test_saw() {
        assert!(saw(1.0, 0.0).approx_eq_ulps(&0.0, 1));
        assert!(saw(1.0, 0.4999999999999999).approx_eq_ulps(&1.0, 2));
        assert!(saw(1.0, 0.5).approx_eq_ulps(&-1.0, 1));
        assert!(saw(1.0, 1.5).approx_eq_ulps(&-1.0, 1));
        assert!(saw(1.0, 0.25).approx_eq_ulps(&0.5, 1));
    }

    #[test]
    fn test_triangle() {
        assert!(triangle(1.0, 0.0).approx_eq_ulps(&1.0, 1));
        assert!(triangle(1.0, 0.5).approx_eq_ulps(&-1.0, 1));
        assert!(triangle(1.0, 1.0).approx_eq_ulps(&1.0, 1));
    }
}
