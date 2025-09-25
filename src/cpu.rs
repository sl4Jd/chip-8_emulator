pub struct CPU {
    // CPU fields here
    registers: [u8; 16],
    pc: u16,
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<u16>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 16],
            pc: 0x200,
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: Vec::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        // initialization logic
        Ok(())
    }
}