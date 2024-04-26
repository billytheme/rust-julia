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

fn calc_pixel(pixel: (u32, u32), dimension: u32) -> (u8, u8, u8) {
    // Display is between -2, 2 on both axes. Determine the size of each pixel, then get a value

    let f64_dimension: f64 = dimension.into();

    let pixel_size = 4.0 / f64_dimension;

    let equivalent_imaginary = Imaginary {
        real: (pixel.0 as f64 * pixel_size) - 2.0,
        i: (pixel.1 as f64 * pixel_size) - 2.0,
    };

    match calc_escape(50, equivalent_imaginary) {
        EscapeResult::Bounded => (0, 0, 0),
        EscapeResult::Escaped(_) => (255, 255, 255),
    }
}

pub fn draw(frame: &mut [u8], width: u32, height: u32) {
    for (idx, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let idx_signed = idx as i32;
        let width_signed = width as i32;
        let height_signed = height as i32;
        let x = idx_signed % width_signed;
        let y = idx_signed / width_signed;

        let (r, g, b) = calc_pixel((x as u32, y as u32), width);
        pixel.copy_from_slice(&[r, g, b, 255])
    }
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
