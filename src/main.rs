use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::env;
use std::process;
use sdl2::Sdl;

mod cpu;
use cpu::CPU;

static mut CPU_INSTANCE: Option<Box<CPU>> = None;

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
    unsafe {
        CPU_INSTANCE = Some(cpu);
    }
}

