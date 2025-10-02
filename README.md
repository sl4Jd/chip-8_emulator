# Chip-8 emulator
Chip-8 emulator I made as first Rust project
## REQUIREMENTS
just sdl2
### Windows
you can find binaries here: https://github.com/libsdl-org/SDL/releases
### Arch
```bash
sudo pacman -S sdl2
```
### Ubuntu
```bash
sudo apt update
sudo apt install libsdl2-dev
```
### Fedora
```bash
sudo dnf install SDL2-devel
```
## CLONE
```bash
git clone https://github.com/sl4Jd/chip-8_emulator.git
```
```bash
cd chip-8_emulator
```
```bash
git submodule update --init --recursive 
```
## BUILD AND RUN
```bash
cargo run "path/to/rom"
```
Example
```bash
cargo run "chip8_roms/games/Pong (1 player).ch8"
```
