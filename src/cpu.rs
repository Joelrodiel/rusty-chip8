use crate::instructions::*;

use std::fs::File;
use std::io::Read;

pub struct Cpu {
    pub opcode: u16,
    pub v: [u8; 16],
    pub i: u16,
    pub sound_timer: u8,
    pub delay_timer: u8,
    pub pc: usize,
    pub sp: usize,
    pub memory: [u8; 4096],
    pub stack: [u16; 16],
    pub keys: [bool; 16],
    pub pixels: [[bool; 64]; 32]
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            opcode: 0,
            v: [0; 16],
            i: 0x200,
            sound_timer: 0,
            delay_timer: 0,
            pc: 0x200,
            sp: 0,
            memory: [0; 4096],
            stack: [0; 16],
            keys: [false; 16],
            pixels: [[false; 64]; 32]
        }
    }

    pub fn load_program(&mut self, file_name: &str) {
        let data = &mut vec![0; 0x200];
        let mut program_file = File::open(file_name).expect("Game was not found!");
        let mut buffer = [0; 3584];
        let buffer_size = program_file.read(&mut buffer[..]).expect("Error reading file!");

        self.load_fontset();

        for byte in buffer.iter() {
            data.push(*byte);
        }

        for (index, &byte) in data.iter().enumerate() {
            self.memory[index] = byte;
        }
    }

    pub fn execute_cycle(&mut self) {
        self.fetch_opcode();
        self.execute_opcode();
        self.update_timers();
    }

    fn load_fontset(&mut self) {
        let fontset: [u8; 80] = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for i in 0..80 {
            self.memory[i] = fontset[i];
        }
    }

    pub fn fetch_opcode(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
        println!("Opcode fetched: {:X}", self.opcode);
    }

    pub fn execute_opcode(&mut self) {
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x0FFF {
                0x00E0 => cls(self),
                0x00EE => ret(self),
                _      => panic!("Wot? What is {:X}???", self.opcode)
            },
            0x1000 => jp(self),
            0x2000 => call(self),
            0x3000 => se_vx_kk(self),
            0x4000 => sne_vx_kk(self),
            0x5000 => se_vx_vy(self),
            0x6000 => ld_vx_kk(self),
            0x7000 => add_vx_kk(self),
            0x8000 => match self.opcode & 0x000F {
                0x0000 => ld_vx_vy(self),
                0x0001 => or_vx_vy(self),
                0x0002 => and_vx_vy(self),
                0x0003 => xor_vx_vy(self),
                0x0004 => add_vx_vy(self),
                0x0005 => sub_vx_vy(self),
                0x0006 => shr_vx_vy(self),
                0x0007 => subn_vx_vy(self),
                0x000E => shl_vx_vy(self),
                _      => panic!("Wot? What is {:X}???", self.opcode)
            },
            0x9000 => sne_vx_vy(self),
            0xA000 => ld_i_nnn(self),
            0xB000 => jp_v0_nnn(self),
            0xC000 => rnd_vx_kk(self),
            0xD000 => drw_vx_vy_n(self),
            0xE000 => match self.opcode & 0x00FF {
                0x009E => skp_vx(self),
                0x00A1 => sknp_vx(self),
                _      => panic!("Wot? What is {:X}???", self.opcode)
            },
            0xF000 => match self.opcode & 0x00FF {
                0x0007 => ld_vx_dt(self),
                0x000A => ld_vx_k(self),
                0x0015 => ld_dt_vx(self),
                0x0018 => ld_st_vx(self),
                0x001E => add_i_vx(self),
                0x0029 => ld_f_vx(self),
                0x0033 => ld_b_vx(self),
                0x0055 => ld_i_vx(self),
                0x0065 => ld_vx_i(self),
                _      => panic!("Wot? What is {:X}???", self.opcode)
            },
            _ => panic!("Wot? What is {:X}???", self.opcode)
        }
    }

    fn update_timers(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn inc_pc(&mut self) { self.pc += 2; }

    pub fn get_nnn(&self) -> u16 { self.opcode & 0x0FFF }
    pub fn get_kk(&self) -> u8 { (self.opcode & 0x00FF) as u8 }
    pub fn get_x(&self) -> u8 { ((self.opcode & 0x0F00) >> 8) as u8 }
    pub fn get_y(&self) -> u8 { ((self.opcode & 0x00F0) >> 8) as u8 }
    pub fn get_n(&self) -> u8 { (self.opcode & 0x000F) as u8 }
}
