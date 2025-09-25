use std::time::Duration;
use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

mod cpu;
use cpu::CPU;

static mut SDL_CONTEXT: Option<Sdl> = None;
static mut CANVAS: Option<Canvas<Window>> = None;
static mut TEXTURE: Option<Texture<'static>> = None;

fn init(){
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
    let texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, 64, 32)
        .unwrap();
    unsafe{
        SDL_CONTEXT = Some(sdl_context);
        CANVAS = Some(canvas);
        TEXTURE = Some(std::mem::transmute::<Texture<'_>, Texture<'static>>(texture));
    }
}
fn deinit() {
    unsafe{
        CANVAS = None; 
        TEXTURE = None;
        SDL_CONTEXT = None;
    }
}

const KEYMAP: [sdl2::keyboard::Scancode; 16] = [
    sdl2::keyboard::Scancode::X,
    sdl2::keyboard::Scancode::Num1,
    sdl2::keyboard::Scancode::Num2,
    sdl2::keyboard::Scancode::Num3,
    sdl2::keyboard::Scancode::Q,
    sdl2::keyboard::Scancode::W,
    sdl2::keyboard::Scancode::E,
    sdl2::keyboard::Scancode::A,
    sdl2::keyboard::Scancode::S,
    sdl2::keyboard::Scancode::D,
    sdl2::keyboard::Scancode::Z,
    sdl2::keyboard::Scancode::C,
    sdl2::keyboard::Scancode::Num4,
    sdl2::keyboard::Scancode::R,
    sdl2::keyboard::Scancode::F,
    sdl2::keyboard::Scancode::V,
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
    init();

    // Initialize CPU
    let mut cpu = Box::new(CPU::new());
    // Load ROM
    if let Err(err) = load_rom(&filename, &mut cpu) {
        eprintln!("Failed to load ROM: {}", err);
        return;
    }
}


