extern crate rand;
extern crate piston_window;

mod cpu;
mod instructions;

use cpu::Cpu;

use piston_window::*;

const SCALE: usize = 2;

fn main() {
    let cpu = &mut Cpu::new();
    cpu.load_program("roms/pong.ch8");

    let mut window: PistonWindow = WindowSettings::new("Chip 8", [800, 600]).exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        cpu.execute_cycle();
        
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            for y in 0..32 {
                for x in 0..64 {
                    if cpu.pixels[y][x] {
                        rectangle([1.0, 0.0, 0.0, 1.0],
                                  [(x * SCALE) as f64, (y * SCALE) as f64, SCALE as f64, SCALE as f64],
                                  context.transform,
                                  graphics);
                    }
                }
            }
        });
    }

}
