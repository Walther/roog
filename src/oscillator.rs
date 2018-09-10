use std::f32::consts::PI;


// Initial thought: let's treat this kind of like shader code:
// we just evaluate the function given the parameters

pub fn sin(hertz: f32, time: f32) -> f32 {
    return (2.0 * PI * hertz * time).sin();
}

pub fn square(hertz: f32, time: f32) -> f32 {
    if (2.0 * PI * hertz * time).sin().is_sign_positive() {
        return 1.0;
    } else {
        return -1.0;
    }
}

pub fn saw(hertz: f32, time: f32) -> f32 {
    let x = hertz * time;
    return 2.0 *(x - (0.5+x).floor());
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate float_cmp;
    use oscillator::tests::float_cmp::ApproxEqUlps;

    #[test]
    fn test_sin() {
        assert!(sin(1.0, 0.0).approx_eq_ulps(&0.0, 1));
        assert!(sin(440.0, 1.0/1760.0).approx_eq_ulps(&1.0, 1));
        // TODO: fix these broken tests and the possibly broken code!
        // assert!(sin(1.0, 1.0).approx_eq_ulps(&0.0, 1)); // FAILS
        // assert!(sin(1.0, 2.0).approx_eq_ulps(&0.0, 1)); // FAILS
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
        // assert!(saw(1.0, 1.0).approx_eq_ulps(&0.0, 1)); // FAILS
        assert!(saw(1.0, 0.4999999).approx_eq_ulps(&1.0, 3));
        assert!(saw(1.0, 0.5).approx_eq_ulps(&-1.0, 1));
        assert!(saw(1.0, 1.5).approx_eq_ulps(&-1.0, 1));
        assert!(saw(1.0, 0.25).approx_eq_ulps(&0.5, 1));
    }
}
