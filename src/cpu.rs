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
    pub fn emulate_cycle(&mut self) {
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
                let addr = self.opcode & 0x0FFF;
                self.pc = addr;
            }
            0x2000 => {
                // Call subroutine at NNN
                let addr = self.opcode & 0x0FFF;
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = addr;
            }
            0x3000 => {
                // Skip next instruction if Vx == NN
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                let nn = (self.opcode & 0x00FF) as u8;
                if self.registers[x] == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            // More opcodes to be implemented...
            _ => {
                println!("Unknown opcode: {:#X}", self.opcode);
                self.pc += 2;
            }
        }

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // Beep sound can be implemented here
            }
            self.sound_timer -= 1;
        }
    }
} 