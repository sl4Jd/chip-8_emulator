use std::time::Duration;
use std::thread;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::render::TextureCreator;
use sdl2::render::{Canvas, Texture};
use sdl2::rect::Rect;
use sdl2::video;
use sdl2::video::Window;
use sdl2::keyboard::Scancode;
use sdl2::Sdl;
use sdl2::audio::AudioCallback;

mod cpu;
use cpu::CPU;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: i16,
}

impl AudioCallback for SquareWave {
    type Channel = i16; // mono output

    fn callback(&mut self, out: &mut [i16]) {
        for x in out.iter_mut() {
            *x = if self.phase < 0.5 { self.volume } else { -self.volume };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

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

    // Audio
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = sdl2::audio::AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), // mono
        samples: None,
    };

    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 3000,
            }
        })
        .unwrap();

    // CPU and Timer frequencies
    let cycle_duration = Duration::from_secs_f64(1.0 / 500 as f64);
    let timer_duration = Duration::from_secs_f64(1.0 / 60 as f64);

    let mut last_cycle_time = Instant::now();
    'run: loop {
        let cycle = Instant::now();
        // Emulate cpu cycle
        if !cpu.emulate_cycle() {
            break 'run;  // stop loop when emulate_cycle signals end
        }

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

        // Update at 60Hz
        if last_cycle_time.elapsed() >= timer_duration {
            // Update timers
            if cpu.delay_timer > 0 {
                cpu.delay_timer -= 1;
            }
            if cpu.sound_timer > 0 {
                device.resume(); 
                cpu.sound_timer -= 1;
            }
            else {device.pause();}

            last_cycle_time = Instant::now();

            // Render graphics
            canvas.clear();
            // Update texture with current graphics state
            build_texture(&cpu, &mut texture);

            // Destination rectangle (scale CHIP-8 64x32 to 512x256)
            let dest = Rect::new(0, 0, 512, 256);

            // Copy the texture to the canvas
            canvas.copy(&texture, None, Some(dest)).unwrap();

            // Present the updated frame
            canvas.present();
        }

        let elapsed = cycle.elapsed();
        if elapsed < cycle_duration {
            thread::sleep(cycle_duration - elapsed);
        }
    }
}


