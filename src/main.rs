use display::{HEIGHT, WIDTH};
use minifb::{Window, WindowOptions};

use crate::cpu::Cpu;
use std::env;

mod cpu;
mod display;

const SCALE: usize = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let game = &args[1];

    let window = Window::new(
        &format!("CHIP-8 - {}", game),
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .expect("Window failed to open");

    let mut cpu = Cpu::new(window);
    cpu.load_rom(game);

    while cpu.is_running() {
        cpu.step();
    }
}
