use crate::registers::Registers;

pub struct CPU {
  registers: Registers,
  pc: u16, // Program counter -> 2 bytes
  sp: u16, // Stack Pointer -> 2 bytes
}

impl CPU {
  pub fn new() -> CPU {
    CPU {
      registers: Registers::new(),
      pc: 0x0000,
      sp: 0x0000,
    }
  }

  pub fn execute_cycle(&mut self) {
    println!("executing instruction");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let cpu = CPU::new();

    assert_eq!(cpu.pc, 0x0000, "Initialize PC");
    assert_eq!(cpu.sp, 0x0000, "Initialize SP");
  }
}
