use rand::random;

pub const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct CPU {
    // CPU fields here
    pub opcode: u16,
    pub memory: [u8; 4096],
    pub graphics: [u8; 64 * 32],
    pub registers: [u8; 16],
    pub index: u16,
    pub pc: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub sp: u8,
    pub keys: [u8; 16],


}
impl CPU {
    pub fn new() -> Self {
        let mut cpu = CPU {
            opcode: 0,
            memory: [0; 4096],
            graphics: [0; 64 * 32],
            registers: [0; 16],
            index: 0,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keys: [0; 16],
        };
        cpu.memory[0..FONTSET.len()].copy_from_slice(&FONTSET);
        cpu
    }
    pub fn emulate_cycle(&mut self) -> bool {
        if self.pc > 0xFFF {
            return false;
        }

        // Fetch Opcode
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[(self.pc + 1) as usize] as u16);
        
        // Decode and Execute Opcode
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x00FF {
                0x00E0 => {
                    // Clear the display
                    self.graphics = [0; 64 * 32];
                    self.pc += 2;
                }
                0x00EE => {
                    // Return from subroutine
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                    self.pc += 2;
                }
                _ => {
                    println!("Unknown opcode: {:#X}", self.opcode);
                    self.pc += 2;
                }
            },
            0x1000 => {
                // Jump to address NNN
                self.pc = self.opcode & 0x0FFF;
            }
            0x2000 => {
                // Call NNN, stack pushed
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }
            0x3000 => {
                // Skip next instruction if Vx == NN
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                if self.registers[x] == (self.opcode & 0x00FF) as u8 {
                    self.pc += 2;
                } 
                self.pc += 2;
            }
            0x4000 => {
                // Skip next instruction if Vx != NN
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                if self.registers[x] != (self.opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x5000 => {
                // Skip next instruction if Vx == Vy
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                let y = ((self.opcode & 0x00F0) >> 4) as usize;
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x6000 => {
                // Set Vx = NN
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                self.registers[x] = (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }
            0x7000 => {
                // Set Vx = Vx + NN
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                self.registers[x] = self.registers[x].wrapping_add((self.opcode & 0x00FF) as u8);
                self.pc += 2;
            }
            0x8000 => {
                // ALU operations
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                let y = ((self.opcode & 0x00F0) >> 4) as usize;
                match self.opcode & 0x000F {
                    0 => {
                        // Set Vx = Vy
                        self.registers[x] = self.registers[y];
                    }
                    1 => {
                        // Set Vx = Vx OR Vy
                        self.registers[x] |= self.registers[y];
                    }
                    2 => {
                        // Set Vx = Vx AND Vy
                        self.registers[x] &= self.registers[y];
                    }
                    3 => {
                        // Set Vx = Vx XOR Vy
                        self.registers[x] ^= self.registers[y];
                    }
                    4 => {
                        // Set Vx = Vx + Vy, set VF = carry
                        let (sum, carry) = self.registers[x].overflowing_add(self.registers[y]);
                        self.registers[x] = sum;
                        self.registers[0xF] = if carry { 1 } else { 0 };
                    }
                    5 => {
                        // Set Vx = Vx - Vy, set VF = NOT borrow
                        let (diff, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
                        self.registers[x] = diff;
                        self.registers[0xF] = if borrow { 0 } else { 1 };
                    }
                    6 => {
                        // Set Vx = Vx shift right 1
                        self.registers[0xF] = self.registers[x] & 0x1;
                        self.registers[x] >>= 1;
                    }
                    7 => {
                        // Set Vx = Vy - Vx, set VF = NOT borrow
                        let (diff, borrow) = self.registers[y].overflowing_sub(self.registers[x]);
                        self.registers[x] = diff;
                        self.registers[0xF] = if borrow { 0 } else { 1 };
                    }
                    0xE => {
                        // Set Vx = Vx shift left 1
                        self.registers[0xF] = (self.registers[x] & 0x80) >> 7;
                        self.registers[x] <<= 1;
                    }
                    _ => {
                        println!("Unknown opcode: {:#X}", self.opcode);
                    }
                }
                self.pc += 2;
            }
            0x9000 => {
                // Skip next instruction if Vx != Vy
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                let y = ((self.opcode & 0x00F0) >> 4) as usize;
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0xA000 => {
                // Set I = NNN
                self.index = self.opcode & 0x0FFF;
                self.pc += 2;
            }
            0xB000 => {
                // Jump to address NNN + V0
                self.pc = (self.opcode & 0x0FFF) + self.registers[0] as u16;
            }
            0xC000 => {
                // Set Vx = random byte AND NN
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                let nn = (self.opcode & 0x00FF) as u8;
                let rand_byte: u8 = random();
                self.registers[x] = rand_byte & nn;
                self.pc += 2;
            }
            0xD000 => {
                // Draw n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
                let x = self.registers[((self.opcode & 0x0F00) >> 8) as usize] as u16;
                let y = self.registers[((self.opcode & 0x00F0) >> 4) as usize] as u16;
                let height = (self.opcode & 0x000F) as u16;
                self.registers[0xF] = 0;

                for y_vert in 0..height {
                    let pixel = self.memory[(self.index + y_vert) as usize];
                    for x_vert in 0..8 {
                        if (pixel & (0x80 >> x_vert)) != 0 {
                            let x_coord = (x + x_vert) % 64;
                            let y_coord = (y + y_vert) % 32;
                            let index = (x_coord + (y_coord * 64)) as usize;
                            if self.graphics[index] == 1 {
                                // Collision detected
                                self.registers[0xF] = 1;
                            }
                            self.graphics[index] ^= 1;
                        }
                    }
                }
                self.pc += 2;
            }
            0xE000 => {
                // Miscellaneous opcodes
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                match self.opcode & 0x00FF {
                    0x9E => {
                        // Skip next instruction if key with the value of Vx is pressed
                        if self.keys[self.registers[x] as usize] != 0 {
                            self.pc += 2;
                        }
                        self.pc += 2;
                    }
                    0xA1 => {
                        // Skip next instruction if key with the value of Vx is not pressed
                        if self.keys[self.registers[x] as usize] == 0 {
                            self.pc += 2;
                        }
                        self.pc += 2;
                    }
                    _ => {
                        println!("Unknown opcode: {:#X}", self.opcode);
                        self.pc += 2;
                    }
                }
            }
            0xF000 => {
                // also miscellaneous opcodes
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                let m = self.opcode & 0x00FF;

                if m == 0x07 {
                    self.registers[x] = self.delay_timer;
                } else if m == 0x0A {
                    // Store the value of the key in Vx if pressed
                    let mut key_pressed = false;
                    for (i, &key) in self.keys.iter().enumerate() {
                        if key != 0 {
                            self.registers[x] = i as u8;
                            key_pressed = true;
                            break;
                        }
                    }
                    if !key_pressed {
                        return true; // Skip this cycle and wait for a key press
                    }
                } else if m == 0x15 {
                    self.delay_timer = self.registers[x];
                } else if m == 0x18 {
                    self.sound_timer = self.registers[x];
                } else if m == 0x1E {
                    // I = I + Vx
                    self.index = self.index.wrapping_add(self.registers[x] as u16);
                } else if m == 0x29 {
                    // I = location of sprite for digit Vx
                    self.index = (self.registers[x] as u16) * 5;
                } else if m == 0x33 {
                    // Store BCD representation of Vx in memory locations I, I+1, and I+2
                    let value = self.registers[x];
                    self.memory[self.index as usize] = value / 100;
                    self.memory[(self.index + 1) as usize] = (value / 10) % 10;
                    self.memory[(self.index + 2) as usize] = value % 10;
                } else if m == 0x55 {
                    // Store registers V0 through Vx in memory starting at location I
                    for i in 0..=x {
                        self.memory[(self.index + i as u16) as usize] = self.registers[i];
                    }
                } else if m == 0x65 {
                    // Read registers V0 through Vx from memory starting at location I
                    for i in 0..=x {
                        self.registers[i] = self.memory[(self.index + i as u16) as usize];
                    }
                } else {
                    println!("Unknown opcode: {:#X}", self.opcode);
                }
                self.pc += 2;
            }
            _ => {
                println!("Unknown opcode: {:#X}", self.opcode);
                self.pc += 2;
            }
        }
        true
    }
} 