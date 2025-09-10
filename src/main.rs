use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() -> Result<(), String> {
    // initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // create window
    let window = video_subsystem
        .window("Hello SDL2 (Rust)", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // create renderer
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } 
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // draw a white rectangle
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = canvas.fill_rect(sdl2::rect::Rect::new(200, 150, 240, 180));

        // show frame
        canvas.present();

        // cap frame rate ~60fps
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

