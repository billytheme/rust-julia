mod imaginary;
mod julia;

use color_eyre::eyre::Result;
use imaginary::Imaginary;
use pixels::{Pixels, SurfaceTexture};
use tracing::error;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let event_loop = EventLoop::new()?;
    let mut input = WinitInputHelper::new();

    // let window = WindowBuilder::new().build(&event_loop)?;

    // let mut pixels = {
    //     let window_size = window.inner_size();
    //     let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    //     Pixels::new(WIDTH, HEIGHT, surface_texture)?
    // };

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Conway's Game of Life")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, control_flow| {
        // Draw the current frame
        if let Event::WindowEvent {
            window_id: _,
            event: WindowEvent::RedrawRequested,
        } = event
        {
            draw(pixels.frame_mut(), WIDTH, HEIGHT);
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                control_flow.exit();
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                control_flow.exit();
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    control_flow.exit();
                    return;
                }
            }

            // Update internal state and request a redraw
            // world.update();
            window.request_redraw();
        }
    })?;

    println!("Hello, world!");
    Ok(())
}

fn draw(frame: &mut [u8], width: u32, height: u32) {
    for (idx, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let idx_signed = idx as i32;
        let width_signed = width as i32;
        let height_signed = height as i32;
        let x = idx_signed % width_signed;
        let y = idx_signed / width_signed;

        let (r, g, b) = julia::calc_pixel(
            (x as u32, y as u32),
            width,
            3.0,
            Imaginary { real: 2.0, i: 2.0 },
        );
        pixel.copy_from_slice(&[r, g, b, u8::MAX])
    }
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    // for source in err.sources().skip(1) {
    //     error!("  Caused by: {source}");
    // }
}
