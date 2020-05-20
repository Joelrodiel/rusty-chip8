use crate::cpu::Cpu;
use crate::rand::Rng;

// 0x00E0: CLS; Clear the display.
pub fn cls(cpu: &mut Cpu) {
    for y in 0..32 {
        for x in 0..64 {
            cpu.pixels[y as usize][x as usize] = false;
        }
    }

    cpu.inc_pc();
}

// 0x00EE: RET; Return from subroutine.
pub fn ret(cpu: &mut Cpu) {
    cpu.pc = cpu.stack[cpu.sp] as usize;
    cpu.sp -= 1;
}

// 0x1nnn: JP nnn; Jump to address nnn.
pub fn jp(cpu: &mut Cpu) {
    cpu.pc = (cpu.opcode & 0x0FFF) as usize;
}

// 0x2nnn: CALL nnn; Call subroutine at nnn.
pub fn call(cpu: &mut Cpu) {
    cpu.sp += 1;
    cpu.stack[cpu.sp] = cpu.pc as u16;
    cpu.pc = cpu.get_nnn() as usize;
}

// 0x3xkk: SE Vx, kk; Skip next instruction if Vx == kk.
pub fn se_vx_kk(cpu: &mut Cpu) {
    if cpu.v[cpu.get_x() as usize] == cpu.get_kk() {
        cpu.inc_pc();
    }

    cpu.inc_pc();
}

// 0x4xkk: SNE Vx, kk; Skip next instruction if Vx != kk.
pub fn sne_vx_kk(cpu: &mut Cpu) {
    if cpu.v[cpu.get_x() as usize] != cpu.get_kk() {
        cpu.inc_pc();
    }

    cpu.inc_pc();
}

// 0x5xy0: SE Vx, Vy; Skip next instruction if Vx == Vy.
pub fn se_vx_vy(cpu: &mut Cpu) {
    if cpu.v[cpu.get_x() as usize] == cpu.v[cpu.get_y() as usize] {
        cpu.inc_pc();
    }

    cpu.inc_pc();
}

// 6xkk: LD Vx, kk; Set Vx = kk.
pub fn ld_vx_kk(cpu: &mut Cpu) {
    cpu.v[cpu.get_x() as usize] = cpu.get_kk();

    cpu.inc_pc();
}

// 7xkk: ADD Vx, kk; Set Vx += kk.
pub fn add_vx_kk(cpu: &mut Cpu) {
    cpu.v[cpu.get_x() as usize] += cpu.get_kk();

    cpu.inc_pc();
}

// 8xy0: LD Vx, Vy; Set Vx = Vy.
pub fn ld_vx_vy(cpu: &mut Cpu) {
    cpu.v[cpu.get_x() as usize] = cpu.v[cpu.get_y() as usize];

    cpu.inc_pc();
}

// 8xy1: OR Vx, Vy; Set Vx = Vx | Vy.
pub fn or_vx_vy(cpu: &mut Cpu) {
    let x = cpu.get_x() as usize;
    cpu.v[x] = cpu.v[x] | cpu.v[cpu.get_y() as usize];

    cpu.inc_pc();
}

// 8xy2: AND Vx, Vy; Set Vx = Vx & Vy.
pub fn and_vx_vy(cpu: &mut Cpu) {
    let x = cpu.get_x() as usize;
    cpu.v[x] = cpu.v[x] & cpu.v[cpu.get_y() as usize];

    cpu.inc_pc();
}

// 8xy3: XOR Vx, Vy; Set Vx = Vx & Vy.
pub fn xor_vx_vy(cpu: &mut Cpu) {
    let x = cpu.get_x() as usize;
    cpu.v[x] = cpu.v[x] ^ cpu.v[cpu.get_y() as usize];

    cpu.inc_pc();
}

// 8xy4: ADD Vx, Vy; Set Vx += Vy.
pub fn add_vx_vy(cpu: &mut Cpu) {
    let x = cpu.get_x() as usize;
    let sum = (cpu.v[x] as u16) + (cpu.v[cpu.get_y() as usize] as u16);
    
    if sum > 0xFF {
        cpu.v[0xF] = 1;
    } else {
        cpu.v[0xF] = 0;
    }

    cpu.v[x] = sum as u8;

    cpu.inc_pc();
}

// 8xy5: SUB Vx, Vy; Set Vx -= Vy.
pub fn sub_vx_vy(cpu: &mut Cpu) {
    let x = cpu.get_x() as usize;
    let y = cpu.get_y() as usize;
    
    if cpu.v[x] > cpu.v[y] {
        cpu.v[0xF] = 1;
    } else {
        cpu.v[0xF] = 0;
    }

    cpu.v[x] = cpu.v[x].wrapping_sub(cpu.v[y]);

    cpu.inc_pc();
}

// 8xy6: SHR Vx {, Vy}; Set Vx = Vx SHR 1.
pub fn shr_vx_vy(cpu: &mut Cpu) {
    let x = cpu.get_x() as usize;
    
    cpu.v[0xF] = cpu.v[x] & 1;

    cpu.v[x] >>= 1;

    cpu.inc_pc();
}

// 8xy7: SUBN Vx, Vy; Set Vx = Vy - Vx.
pub fn subn_vx_vy(cpu: &mut Cpu) {
    let x = cpu.get_x() as usize;
    let y = cpu.get_y() as usize;
    
    if cpu.v[y] > cpu.v[x] {
        cpu.v[0xF] = 1;
    } else {
        cpu.v[0xF] = 0;
    }

    cpu.v[x] = cpu.v[y].wrapping_sub(cpu.v[x]);

    cpu.inc_pc();
}

// 8xyE: SHL Vx {, Vy}; Set Vx = Vx SHL 1.
pub fn shl_vx_vy(cpu: &mut Cpu) {
    let x = cpu.get_x() as usize;
    
    cpu.v[0xF] = cpu.v[x] >> 7;

    cpu.v[x] <<= 1;

    cpu.inc_pc();
}

// 0x9xy0: SNE Vx, Vy; Skip next instruction Vx != Vy.
pub fn sne_vx_vy(cpu: &mut Cpu) {
    if cpu.v[cpu.get_x() as usize] != cpu.v[cpu.get_y() as usize] {
        cpu.inc_pc();
    }

    cpu.inc_pc();
}

// 0xAnnn: LD I, nnn; Set I = nnn.
pub fn ld_i_nnn(cpu: &mut Cpu) {
    cpu.i = cpu.get_nnn();

    cpu.inc_pc();
}

// 0xBnnn: JP V0, nnn; Jump to location nnn + V0.
pub fn jp_v0_nnn(cpu: &mut Cpu) {
    cpu.pc = (cpu.get_nnn() + (cpu.v[0] as u16)) as usize;

    cpu.inc_pc();
}

// 0xCxkk: RND Vx, kk; Set Vx = rand byte AND kk.
pub fn rnd_vx_kk(cpu: &mut Cpu) {
    let mut rng = rand::thread_rng();

    let rn: u8 = rng.gen();
    cpu.v[cpu.get_x() as usize] = rn & cpu.get_kk();

    cpu.inc_pc();
}

// 0xDxyn: DRW Vx, Vy, n; Display n-byte sprite at mem addrs I at (Vx, Vy), Set Vf collision.
pub fn drw_vx_vy_n(cpu: &mut Cpu) {
    let vx   = cpu.v[cpu.get_x() as usize];
    let vy   = cpu.v[cpu.get_y() as usize];
    let rows = cpu.get_n();

    cpu.v[0xF] = 0;
    for byte in 0..rows {
        let sprite = cpu.memory[(cpu.i + (byte as u16)) as usize];

        let y = (vy + byte) % 32;
        for bit in 0..8 {
            let x = (vx + bit) % 64;
            let pixel = (sprite >> (7 - bit)) & 1;
            cpu.v[0xF] |= pixel & (cpu.pixels[y as usize][x as usize] as u8);
            cpu.pixels[y as usize][x as usize] ^= pixel != 0;
        }
    }

    cpu.inc_pc();
}

// 0xEx9E: SKP Vx; Skip next instruction if key Vx pressed.
pub fn skp_vx(cpu: &mut Cpu) {
    if cpu.keys[cpu.v[cpu.get_x() as usize] as usize] {
        cpu.inc_pc();
    }

    cpu.inc_pc();
}

// 0xExA1: SKNP Vx; Skip next instruction if key Vx not pressed.
pub fn sknp_vx(cpu: &mut Cpu) {
    if !cpu.keys[cpu.v[cpu.get_x() as usize] as usize] {
        cpu.inc_pc();
    }

    cpu.inc_pc();
}

// 0xFx07: LD Vx, DT; Set Vx = Delay timer.
pub fn ld_vx_dt(cpu: &mut Cpu) {
    cpu.v[cpu.get_x() as usize] = cpu.delay_timer;
    
    cpu.inc_pc();
}

// 0xFx0A: LD Vx, k; Wait for key press, store key in Vx.
pub fn ld_vx_k(cpu: &mut Cpu) {
    for i in 0..16 {
        if cpu.keys[i] {
            cpu.v[cpu.get_x() as usize] = i as u8;
            cpu.inc_pc();
            break;
        }
    }
}

// 0xFx15: LD DT, Vx; Set Delay timer = Vx.
pub fn ld_dt_vx(cpu: &mut Cpu) {
    cpu.delay_timer = cpu.v[cpu.get_x() as usize];

    cpu.inc_pc();
}

// 0xFx18: LD ST, Vx; Set Sound timer = Vx.
pub fn ld_st_vx(cpu: &mut Cpu) {
    cpu.sound_timer = cpu.v[cpu.get_x() as usize];

    cpu.inc_pc();
}

// 0xFx1E: ADD I, Vx; Set I = I + Vx.
pub fn add_i_vx(cpu: &mut Cpu) {
    cpu.i = cpu.i + (cpu.v[cpu.get_x() as usize] as u16);

    cpu.inc_pc();
}

// 0xFx29: LD F, Vx; Set I = location of sprite Vx.
pub fn ld_f_vx(cpu: &mut Cpu) {
    cpu.i = (cpu.v[cpu.get_x() as usize] * 0x5) as u16;

    cpu.inc_pc();
}

// 0xFx33: LD B, Vx; Store BCD of Vx in memory addrs I, I+1, and I+2.
pub fn ld_b_vx(cpu: &mut Cpu) {
    let vx = cpu.v[cpu.get_x() as usize];
    let i  = cpu.i as usize;

    cpu.memory[i] = vx / 100;
    cpu.memory[i+1] = (vx / 10) % 10;
    cpu.memory[i+2] = (vx % 100) % 10;
    
    cpu.inc_pc();
}

// 0xFx55: LD [I], Vx; Store regs V0 through Vx in memory starting at location I.
pub fn ld_i_vx(cpu: &mut Cpu) {
    for i in 0..cpu.get_x() {
        cpu.memory[(cpu.i + (i as u16)) as usize] = cpu.v[i as usize];
    }

    cpu.inc_pc();
}

// 0xFx65: LD Vx, [I]; Read regs V0 through from memory starting at location I.
pub fn ld_vx_i(cpu: &mut Cpu) {
    for i in 0..cpu.get_x() {
        cpu.v[i as usize] = cpu.memory[(cpu.i + (i as u16)) as usize];
    }

    cpu.inc_pc();
}
