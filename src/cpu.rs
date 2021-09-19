use crate::registers::Registers;

pub struct CPU {
    registers: Registers,
    pc: u16, // Program counter -> 2 bytes
    sp: u16, // Stack Pointer -> 2 bytes
}

pub enum Instruction {
    ADD(ArithmeticTarget),
}

pub enum ArithmeticTarget {
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
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow == true;

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

    #[test]
    fn match_instruction_add_c() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x09;
        cpu.registers.c = 0x01;

        cpu.execute_cycle(Instruction::ADD(ArithmeticTarget::C));

        assert_eq!(cpu.registers.a, 0xA, "Result in register A");
    }

    #[test]
    fn add() {
        let mut cpu = CPU::new();
        let value = 0x01;

        cpu.registers.a = 0x09;

        let result = cpu.add(value);

        assert_eq!(result, 0xA, "Result in register A");
        assert_eq!(cpu.registers.f.zero, false, "Zero");
        assert_eq!(cpu.registers.f.subtract, false, "Subtract");
        assert_eq!(cpu.registers.f.half_carry, false, "Half Carry");
        assert_eq!(cpu.registers.f.carry, false, "Carry");
    }
    #[test]
    fn add_half_carry() {
        let mut cpu = CPU::new();
        let value = 0b1;

        cpu.registers.a = 0b10001111;

        let result = cpu.add(value);

        assert_eq!(result, 0b10010000, "Result in register A");
        assert_eq!(cpu.registers.f.zero, false, "Zero");
        assert_eq!(cpu.registers.f.subtract, false, "Subtract");
        assert_eq!(cpu.registers.f.half_carry, true, "Half Carry");
        assert_eq!(cpu.registers.f.carry, false, "Carry");
    }

    #[test]
    fn add_carry_zero() {
        let mut cpu = CPU::new();
        let value = 0x1;

        cpu.registers.a = 0xFF;

        let result = cpu.add(value);

        assert_eq!(result, 0x0, "Result in register A");
        assert_eq!(cpu.registers.f.zero, true, "Zero");
        assert_eq!(cpu.registers.f.subtract, false, "Subtract");
        assert_eq!(cpu.registers.f.half_carry, true, "Half Carry");
        assert_eq!(cpu.registers.f.carry, true, "Carry");
    }
}
