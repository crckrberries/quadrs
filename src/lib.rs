use regex::*;

#[derive(PartialEq, Debug)]
pub enum Root {
    None,
    One(f64),
    Two(f64, f64),
}

pub fn quad(a: f64, b: f64, c: f64) -> Root {
    let delta = b * b - 4.0 * a * c; // discriminant (delta)

    return if delta.is_sign_positive() {
        let plus = (-b + delta.sqrt()) / (2.0 * a);

        if delta != 0.0 {
            // two roots
            let minus = (-b - delta.sqrt()) / (2.0 * a);
            return Root::Two(plus, minus);
        }

        Root::One(plus) // one root
    } else {
        Root::None // no roots
    };
}

pub fn parse(inp: &str) -> [f64; 3] {
    let inp = &*inp.replace(" ", ""); // remove whitespaces from the string so the regex works
    let re = Regex::new(r"[+-]?(\d+)").unwrap();
    let mut factors: Vec<f64> = re
        .find_iter(inp)
        .filter_map(|n| n.as_str().parse().ok())
        .collect::<Vec<f64>>();

    factors.remove(1); // removes the exponent of the first term

    let factors: [f64; 3] = factors.try_into().expect("this isn't a quadratic equation");

    return factors;
}

#[cfg(test)]
mod tests {
    use crate::{parse, quad, Root};

    #[test]
    fn quad_two() {
        // quadratic equation with two roots
        let root = quad(-4.0, -7.0, 12.0); // delta > 0
        assert_eq!(root, Root::Two(-2.8155218370325032, 1.065521837032503));
    }

    #[test]
    fn quad_one() {
        // quadratic equation with one root
        let root = quad(4.0, -12.0, 9.0); // delta = 0
        assert_eq!(root, Root::One(1.5));
    }

    #[test]
    fn quad_none() {
        // quadratic equation with no (real) roots
        let root = quad(1.0, -3.0, 4.0); // delta < 0
        assert_eq!(root, Root::None);
    }

    #[test]
    fn parser() {
        let parsed = parse("3x^2 - 11x - 35");
        assert_eq!(parsed, [3.0, -11.0, -35.0]);
    }
}
