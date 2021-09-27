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

    pub fn execute_cycle(&mut self, opcode: u8) {
        match opcode {
            0x80 => self.registers.a = self.add(self.registers.b),
            0x81 => self.registers.a = self.add(self.registers.c),
            0x82 => self.registers.a = self.add(self.registers.d),
            0x83 => self.registers.a = self.add(self.registers.e),
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
    fn add_a_b() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x09;
        cpu.registers.b = 0x01;

        cpu.execute_cycle(0x80);

        assert_eq!(cpu.registers.a, 0xA, "Result in register A");
    }

    #[test]
    fn add_a_c() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x09;
        cpu.registers.c = 0x01;

        cpu.execute_cycle(0x81);

        assert_eq!(cpu.registers.a, 0xA, "Result in register A");
    }

    #[test]
    fn add_a_d() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x09;
        cpu.registers.d = 0x01;

        cpu.execute_cycle(0x82);

        assert_eq!(cpu.registers.a, 0xA, "Result in register A");
    }

    #[test]
    fn add_a_e() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x09;
        cpu.registers.e = 0x01;

        cpu.execute_cycle(0x83);

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
