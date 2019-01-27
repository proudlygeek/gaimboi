use crate::registers::Registers;

pub struct CPU {
  registers: Registers,
  pc: u16, // Program counter -> 2 bytes
  sp: u16, // Stack Pointer -> 2 bytes
}

enum Instruction {
  ADD(ArithmeticTarget),
}

enum ArithmeticTarget {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
}

impl CPU {
  pub fn new() -> CPU {
    CPU {
      registers: Registers::new(),
      pc: 0x0000,
      sp: 0x0000,
    }
  }

  pub fn execute_cycle(&mut self, instruction: Instruction) {
    match instruction {
      Instruction::ADD(target) => match target {
        ArithmeticTarget::C => {
          let value = self.registers.c;
          let new_value = self.add(value);
          self.registers.a = new_value;
        }
        _ => panic!("not handled"),
      },
      _ => panic!("not handled"),
    }
  }

  fn add(&mut self, value: u8) -> u8 {
    let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
    self.registers.f.zero = new_value == 0;
    self.registers.f.subtract = false;

    new_value
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
