use std::f64::consts::E;

use crate::imaginary::Imaginary;
use rayon::prelude::*;

enum EscapeResult {
    Bounded,
    Escaped {
        iter_count: u32,
        final_val: Imaginary,
    },
}

fn calc_escape(iteration_bound: u32, offset: Imaginary) -> EscapeResult {
    let mut val = Imaginary { real: 0.0, i: 0.0 };
    for iteration in 1..iteration_bound {
        val = &val * &val;
        val = &val + &offset;
        match val {
            _ if val.absolute() > 2.0 => {
                return EscapeResult::Escaped {
                    iter_count: iteration,
                    final_val: val,
                }
            }
            _ => continue,
        }
    }
    EscapeResult::Bounded
}

/// Calculates the RGB value of a pixel
/// pixel: coordinates in image space, between 0 and dimension
/// dimension: the dimensions of the image
/// horizontal_scale: the distance in imaginary units from one side of the dimension to the other. A standard mandlebrot goes from -2 - 2
/// offset: an imaginary number specifying the centre of the image
fn calc_pixel(
    pixel: &(u32, u32),
    dimension: &(u32, u32),
    horizontal_scale: &f64,
    offset: &Imaginary,
) -> (u8, u8, u8) {
    assert!(pixel.0 < dimension.0);
    assert!(pixel.1 < dimension.1);

    let pixel_size = horizontal_scale / dimension.1 as f64;

    let iteration_bound = 500;

    // num_subsamples is in each direction, so the actual number of samples is num_subsamples**2
    let num_subsamples = 2;
    // let mut subsamples = vec![];
    let sub_pixel_size = pixel_size / (num_subsamples + 1) as f64;

    let (r, g, b, num) = (1..=num_subsamples)
        .flat_map(|x_pixel_fraction| {
            (1..=num_subsamples).map(move |y_pixel_fraction| (x_pixel_fraction, y_pixel_fraction))
        })
        .map(|(x_pixel_fraction, y_pixel_fraction)| {
            let equivalent_imaginary = Imaginary {
                real: (pixel.0 as f64 * pixel_size + x_pixel_fraction as f64 * sub_pixel_size)
                    - offset.real * (horizontal_scale / 4.0),
                i: (pixel.1 as f64 * pixel_size + y_pixel_fraction as f64 * sub_pixel_size)
                    - offset.i * (horizontal_scale / 4.0),
            };

            match calc_escape(iteration_bound, equivalent_imaginary) {
                EscapeResult::Bounded => (u8::MIN, u8::MIN, u8::MIN),
                EscapeResult::Escaped {
                    iter_count,
                    final_val,
                } => {
                    // https://stackoverflow.com/questions/369438/smooth-spectrum-for-mandelbrot-set-rendering
                    let smoothed_iters = iter_count as f64
                        - f64::log(f64::log(final_val.absolute(), E), E) / f64::log(2.0, E);
                    let scaled_val = smoothed_iters / iteration_bound as f64;
                    hsv_to_rgb((0.95 + 10.0 * scaled_val, 0.6, 1.0))
                    // (scaled_val, scaled_val, 255)
                }
            }
        })
        .fold((0, 0, 0, 0 as u32), |acc, elem| {
            (
                acc.0 + elem.0 as u32,
                acc.1 + elem.1 as u32,
                acc.2 + elem.2 as u32,
                acc.3 + 1,
            )
        });

    ((r / num) as u8, (g / num) as u8, (b / num) as u8)
}

pub fn calc_frame(
    dimension: &(u32, u32),
    horizontal_scale: &f64,
    offset: &Imaginary,
) -> Vec<(u8, u8, u8)> {
    (0..dimension.0)
        .into_par_iter()
        .flat_map(|y| (0..dimension.1).into_par_iter().map(move |x| (x, y)))
        .map(move |pixel| calc_pixel(&pixel, dimension, horizontal_scale, offset))
        .collect()
}

// https://www.rapidtables.com/convert/color/hsv-to-rgb.html
fn hsv_to_rgb(color: (f64, f64, f64)) -> (u8, u8, u8) {
    // Expects h, s, v to be between 0,1. h values outside this range and wrapped
    let (h, s, v) = color;

    let c = v * s;
    let x = c * (1.0 - f64::abs(((h * 6.0) % 2.0) - 1.0));
    let m = v - c;

    let h1 = ((h % 1.0) * 6.0) as u8;

    let rgb = match h1 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((rgb.0 + m) * 255.0).round() as u8,
        ((rgb.1 + m) * 255.0).round() as u8,
        ((rgb.2 + m) * 255.0).round() as u8,
    )
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
            EscapeResult::Escaped {
                final_val: _,
                iter_count: _,
            } => panic!(),
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
            EscapeResult::Escaped {
                final_val: _,
                iter_count: _,
            } => (),
        }
    }

    #[test]
    fn test_hsv_to_rgb() {
        assert_eq!(hsv_to_rgb((0.0, 0.6, 1.0)), (255, 102, 102));
        assert_eq!(hsv_to_rgb((0.5, 0.6, 1.0)), (102, 255, 255));
        assert_eq!(hsv_to_rgb((0.25, 0.6, 1.0)), (179, 255, 102));
    }
}
