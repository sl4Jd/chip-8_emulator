use std::time::Duration;
use std::thread;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read};
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::render::TextureCreator;
use sdl2::render::{Canvas, Texture};
use sdl2::rect::Rect;
use sdl2::video;
use sdl2::video::Window;
use sdl2::keyboard::Scancode;
use sdl2::Sdl;

mod cpu;
use cpu::CPU;

fn init() -> (Sdl, TextureCreator<video::WindowContext>, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("CHIP-8 Emulator", 512, 256)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .unwrap();
    let texture_creator = canvas.texture_creator();
    (sdl_context, texture_creator, canvas)
}

const KEYMAP: [Scancode; 16] = [
    Scancode::X,
    Scancode::Num1,
    Scancode::Num2,
    Scancode::Num3,
    Scancode::Q,
    Scancode::W,
    Scancode::E,
    Scancode::A,
    Scancode::S,
    Scancode::D,
    Scancode::Z,
    Scancode::C,
    Scancode::Num4,
    Scancode::R,
    Scancode::F,
    Scancode::V,
];

pub fn load_rom<P: AsRef<Path>>(filename: P, cpu: &mut CPU) -> io::Result<()> {
    let mut file = File::open(filename)?;
    println!("Loading ROM!");

    // Get file size
    let size = file.metadata()?.len() as usize;
    println!("ROM File Size: {}", size);

    // Read the ROM
    let mut buffer = Vec::with_capacity(size);
    file.read_to_end(&mut buffer)?;

    // Copy ROM into memory starting at 0x200
    let start = 0x200;
    for (i, &byte) in buffer.iter().enumerate() {
        cpu.memory[start + i] = byte;
    }
    println!("Loading ROM Succeeded!");
    Ok(())
}

pub fn build_texture(system: &CPU, texture: &mut Texture) {
    texture.with_lock(None, |pixels: &mut [u8], _pitch: usize| {
        for y in 0..32 {
            for x in 0..64 {
                let offset = (y * 64 + x) * 4; // 4 bytes per pixel
                let color = if system.graphics[y * 64 + x] == 1 {
                    [0xFF, 0xFF, 0xFF, 0xFF] // white pixel RGBA
                } else {
                    [0x00, 0x00, 0x00, 0xFF] // black pixel RGBA
                };
                pixels[offset..offset + 4].copy_from_slice(&color);
            }
        }
    }).unwrap();
}

fn main() {
    let slow_factor = 1;

    let mut args = env::args();
    args.next(); // skip executable name

    let filename = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("No ROM file given!\n");
            return;
        }
    };

    // Initialize SDL2
    let (sdl_context, texture_creator, mut canvas) = init();
    let mut texture = texture_creator
        .create_texture(PixelFormatEnum::ARGB8888, sdl2::render::TextureAccess::Streaming, 64, 32)
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // Initialize CPU
    let mut cpu = Box::new(CPU::new());
    // Load ROM
    if let Err(err) = load_rom(&filename, &mut cpu) {
        eprintln!("Failed to load ROM: {}", err);
        return;
    }
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'run,

                Event::KeyDown { scancode: Some(sc), .. } => {
                    if sc == Scancode::Escape {
                        break 'run;
                    }

                    // check if sc is in your KEYMAP
                    for (i, &mapped) in KEYMAP.iter().enumerate() {
                        if sc == mapped {
                            cpu.keys[i] = 1;
                            println!("Key {} pressed", i);
                        }
                    }
                }

                Event::KeyUp { scancode: Some(sc), .. } => {
                    for (i, &mapped) in KEYMAP.iter().enumerate() {
                        if sc == mapped {
                            cpu.keys[i] = 0;
                        }
                    }
                }

                _ => {}
            }
        }
        // Emulate cpu cycle
        if !cpu.emulate_cycle() {
            break 'run;  // stop loop when emulate_cycle signals end
        }
    
        canvas.clear();
        // Update texture with current graphics state
        build_texture(&cpu, &mut texture);

        // Destination rectangle (scale CHIP-8 64x32 to 512x256)
        let dest = Rect::new(0, 0, 512, 256);

        // Copy the texture to the canvas
        canvas.copy(&texture, None, Some(dest)).unwrap();

        // Present the updated frame
        canvas.present();

    thread::sleep(Duration::from_millis(16 * slow_factor));
    }
}


