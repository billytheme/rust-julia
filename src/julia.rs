use crate::imaginary::Imaginary;

enum EscapeResult {
    Bounded,
    Escaped(i32),
}

fn calc_escape(iteration_bound: i32, offset: Imaginary) -> EscapeResult {
    let mut val = Imaginary { real: 0.0, i: 0.0 };
    for iteration in 1..iteration_bound {
        val = val * val;
        val = val + offset;
        match val {
            _ if val.absolute() > 2.0 => return EscapeResult::Escaped(iteration),
            _ => continue,
        }
    }
    return EscapeResult::Bounded;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_escape_bounded() {
        match calc_escape(
            50,
            Imaginary {
                real: -0.11,
                i: 0.82,
            },
        ) {
            EscapeResult::Bounded => (),
            EscapeResult::Escaped(_) => panic!(),
        }
    }

    #[test]
    fn test_calc_escape_unbounded() {
        match calc_escape(
            50,
            Imaginary {
                real: -1.15,
                i: 0.58,
            },
        ) {
            EscapeResult::Bounded => panic!(),
            EscapeResult::Escaped(_) => (),
        }
    }
}
