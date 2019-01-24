mod cpu;
mod registers;

use cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    loop {
        cpu.execute_cycle();
    }
}
