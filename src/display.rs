use minifb::Window;
use std::vec;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Display {
    screen: [u8; WIDTH * HEIGHT],
    buffer: Vec<u32>,
}

#[allow(dead_code)]
impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH * HEIGHT],
            buffer: vec![0; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        for point in self.screen.iter_mut() {
            *point = 0;
        }
    }

    pub fn get_point(&self, x: usize, y: usize) -> bool {
        self.screen[x + y * WIDTH] == 1
    }

    fn set_point(&mut self, x: usize, y: usize, on: bool) {
        self.screen[x + y * WIDTH] = on as u8;
    }

    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut collision = false;
        let rows = sprite.len();
        for j in 0..rows {
            let row = sprite[j];
            for i in 0..8 {
                let new_value = row >> (7 - i) & 0x01;
                if new_value == 1 {
                    let x_point = (x as usize + i)  % WIDTH;
                    let y_point = (y as usize + j)  % HEIGHT;
                    let old_value = self.get_point(x_point, y_point);
                    if old_value {
                        collision = true;
                    }
                    self.set_point(x_point, y_point, (new_value == 1) ^ old_value);
                }
            }
        }
        collision
    }

    pub fn render(&mut self, window: &mut Window) {
        for y in 0..HEIGHT {
            let y_coord = y;
            let offset = y * WIDTH;
            for x in 0..WIDTH {
                let point = self.screen[y_coord * WIDTH + x];
                let color = match point {
                    0 => 0x0,
                    1 => 0xFFFFFF,
                    _ => unreachable!(),
                };
                self.buffer[offset + x] = color;
            }
        }

        window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();
    }
}
