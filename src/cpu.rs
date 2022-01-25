use crate::display::Display;
use minifb::{Window, Key, KeyRepeat};
use rand::{prelude::ThreadRng, Rng};
use std::{fs::File, io::Read, time::Instant, time::Duration};

const MEMORY_SIZE: usize = 0x1000;
const PROGRAM_START: u16 = 0x200;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cpu {
    memory: [u8; MEMORY_SIZE],
    display: Display,
    window: Window,
    stack: [u16; 16],
    sp: usize,
    v: [u8; 16],
    i: u16,
    pc: u16,
    delay: u8,
    sound: u8,
    rng: ThreadRng,
    key: Option<Key>,
    mapped_key: Option<u8>,
    last_instruction: Instant,
    last_render: Instant,
    last_input: Instant,
    last_delay: Instant,
}

impl Cpu {
    pub fn new(window: Window) -> Cpu {
        let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];

        let font = [
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
            0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xe0, 0x90, 0x90, 0x90, 0xe0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for (address, value) in font.iter().enumerate() {
            memory[address] = *value;
        }

        Cpu {
            memory,
            display: Display::new(),
            window,
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            sp: 0,
            delay: 0,
            sound: 0,
            rng: rand::thread_rng(),
            key: None,
            mapped_key: None,
            last_instruction: Instant::now(),
            last_render: Instant::now(),
            last_input: Instant::now(),
            last_delay: Instant::now(),
        }
    }

    pub fn step(&mut self) {
        let keys = self.window.get_keys_pressed(KeyRepeat::Yes);
        let mut cur_key = None;

        if !keys.is_empty() {
            cur_key = Cpu::get_mapped_key(&keys[0])
        }

        if cur_key.is_some() || Instant::now() - self.last_input >= Duration::from_millis(200) {
            self.mapped_key = cur_key;
            self.last_input = Instant::now();
        };

        if Instant::now() - self.last_instruction > Duration::from_millis(2) {
            let high: u16 = self.memory[self.pc as usize].into();
            let low: u16 = self.memory[(self.pc + 1) as usize].into();
            let opcode: u16 = (high << 8) | low;

            // println!("{:x}", opcode);

            self.interpret(opcode);
        };

        if Instant::now() - self.last_render > Duration::from_millis(10) {
            self.display.render(&mut self.window);
            self.last_render = Instant::now();
        };
    }

    pub fn is_running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn load_rom(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut rom = Vec::<u8>::new();

        file.read_to_end(&mut rom).unwrap();

        for (address, value) in rom.iter().enumerate() {
            self.memory[address + PROGRAM_START as usize] = *value;
        }
    }

    fn get_mapped_key(key: &Key) -> Option<u8> {
        match key {
            Key::Key1 => Some(0x1),
            Key::Key2 => Some(0x2),
            Key::Key3 => Some(0x3),
            Key::Key4 => Some(0xC),
            Key::Q => Some(0x4),
            Key::W => Some(0x5),
            Key::E => Some(0x6),
            Key::R => Some(0xD),
            Key::A => Some(0x7),
            Key::S => Some(0x8),
            Key::D => Some(0x9),
            Key::F => Some(0xE),
            Key::Z => Some(0xA),
            Key::X => Some(0x0),
            Key::C => Some(0xB),
            Key::V => Some(0xF),
            _ => None,
        }
    }

    fn set_delay(&mut self, delay: u8) {
        self.delay = delay;
        self.last_delay = Instant::now();
    }

    fn get_delay(&self) -> u8 {
        let diff = Instant::now() - self.last_delay;
        let ms = diff.as_millis();
        let ticks = ms / 16;
        if ticks >= self.delay as u128 {
            0
        } else {
            self.delay
        }
    }

    pub fn interpret(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let hi = ((opcode & 0xF000) >> 12) as u8;
        let n = (opcode & 0x000F) as u8;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        let vx = self.v[x];
        let vy = self.v[y];

        self.pc += 2;

        match (hi, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => {
                self.display.clear();
            }
            (0x0, 0x0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp];
            }
            (0x1, _, _, _) => {
                self.pc = nnn;
            }
            (0x2, _, _, _) => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            (0x3, _, _, _) => {
                if vx == kk {
                    self.pc += 2;
                };
            }
            (0x4, _, _, _) => {
                if vx != kk {
                    self.pc += 2;
                };
            }
            (0x5, _, _, 0x0) => {
                if vx == vy {
                    self.pc += 2;
                };
            }
            (0x6, _, _, _) => {
                self.v[x] = kk;
            }
            (0x7, _, _, _) => {
                self.v[x] = vx.wrapping_add(kk);
            }
            (0x8, _, _, 0x0) => {
                self.v[x] = vy;
            }
            (0x8, _, _, 0x1) => {
                self.v[x] = vx | vy;
            }
            (0x8, _, _, 0x2) => {
                self.v[x] = vx & vy;
            }
            (0x8, _, _, 0x3) => {
                self.v[x] = vx ^ vy;
            }
            (0x8, _, _, 0x4) => {
                let (result, overflowed) = vx.overflowing_add(vy);
                match overflowed {
                    true => self.v[0xF] = 1,
                    false => self.v[0xF] = 0,
                };
                self.v[x] = result;
            }
            (0x8, _, _, 0x5) => {
                let (result, overflowed) = vx.overflowing_sub(vy);
                match overflowed {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                };
                self.v[x] = result;
            }
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
            }
            (0x8, _, _, 0x7) => {
                let (result, overflowed) = vy.overflowing_sub(vx);
                match overflowed {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                };
                self.v[x] = result;
            }
            (0x8, _, _, 0xE) => {
                self.v[0xF] = (vx & 0x80) >> 7;
                self.v[x] <<= 1;
            }
            (0x9, _, _, 0x0) => {
                if vx != vy {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            (0xB, _, _, _) => {
                self.pc = nnn + self.v[0] as u16;
            }
            (0xC, _, _, _) => {
                self.v[x] = self.rng.gen_range(0..255) & kk;
            }
            (0xD, _, _, _) => {
                let collision = self.display.draw(
                    vx,
                    vy,
                    &self.memory[self.i as usize..(self.i + n as u16) as usize],
                );
                self.v[0xF] = if collision { 1 } else { 0 };
            }
            (0xE, _, 0x9, 0xE) => {
                if self.mapped_key == Some(vx) {
                    self.pc += 2;
                };
            }
            (0xE, _, 0xA, 0x1) => {
                if self.mapped_key != Some(vx) {
                    self.pc += 2;
                };
            }
            (0xF, _, 0x0, 0x7) => {
                self.v[x] = self.get_delay();
            }
            (0xF, _, 0x0, 0xA) => {
                match self.mapped_key {
                    Some(key_code) => self.v[x] = key_code,
                    None => self.pc -= 2,
                };
            }
            (0xF, _, 0x1, 0x5) => {
                self.set_delay(vx);
            }
            (0xF, _, 0x1, 0x8) => {
                self.sound = vx;
            }
            (0xF, _, 0x1, 0xE) => {
                self.i += vx as u16;
            }
            (0xF, _, 0x2, 0x9) => {
                self.i = vx as u16 * 5;
            }
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.i as usize] = vx / 100;
                self.memory[(self.i + 1) as usize] = (vx % 100) / 10;
                self.memory[(self.i + 2) as usize] = vx % 10;
            }
            (0xF, _, 0x5, 0x5) => {
                for reg in 0..x + 1 {
                    self.memory[self.i as usize + reg] = self.v[reg]
                }
            }
            (0xF, _, 0x6, 0x5) => {
                for reg in 0..x + 1 {
                    self.v[reg] = self.memory[self.i as usize + reg];
                }
            }
            (_, _, _, _) => println!("Invalid opcode: {:x}", opcode),
        };

        self.last_instruction = Instant::now();
    }
}
