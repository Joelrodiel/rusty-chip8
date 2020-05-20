# rusty-chip8
A chip8 emulator written on Rust from scratch.

To play a ROM, edit line 15 in src/main.rs where it says "cpu.load_program(...)" and replace it with the path of the ROM file.
I've put some totally legal ROMs in the repo in the directory named "roms".

Documentation used:
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

Dependecnies:
- rand - Random numebr generator.
- piston_window - GUI Interface.
