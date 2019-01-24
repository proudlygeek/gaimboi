// 8 registers, 8 bit each (A, B, C, D, E, H, L)
pub struct Registers {
  a: u8,
  f: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  h: u8,
  l: u8,
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      a: 0x0,
      f: 0x0,
      b: 0x0,
      c: 0x0,
      d: 0x0,
      e: 0x0,
      h: 0x0,
      l: 0x0,
    }
  }

  fn af(&mut self) -> u16 {
    ((self.a as u16) << 8) | (self.f as u16 & 0xF0)
  }

  fn bc(&mut self) -> u16 {
    (self.b as u16) << 8 | (self.c as u16)
  }

  fn de(&mut self) -> u16 {
    (self.d as u16) << 8 | (self.e as u16)
  }

  fn hl(&mut self) -> u16 {
    (self.h as u16) << 8 | (self.l as u16)
  }

  fn set_af(&mut self, value: u16) {
    self.a = ((value & 0xFF00) >> 8) as u8;
    self.f = (value & 0x00F0) as u8;
  }

  fn set_bc(&mut self, value: u16) {
    self.b = ((value & 0xFF00) >> 8) as u8;
    self.c = (value & 0x00FF) as u8;
  }

  fn set_de(&mut self, value: u16) {
    self.d = ((value & 0xFF00) >> 8) as u8;
    self.e = (value & 0x00FF) as u8;
  }

  fn set_hl(&mut self, value: u16) {
    self.h = ((value & 0xFF00) >> 8) as u8;
    self.l = (value & 0x00FF) as u8;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let r = Registers::new();

    assert_eq!(r.a, 0x00, "Initialize A");
    assert_eq!(r.f, 0x00, "Initialize F");
    assert_eq!(r.b, 0x00, "Initialize B");
    assert_eq!(r.c, 0x00, "Initialize C");
    assert_eq!(r.d, 0x00, "Initialize D");
    assert_eq!(r.e, 0x00, "Initialize E");
    assert_eq!(r.h, 0x00, "Initialize H");
    assert_eq!(r.l, 0x00, "Initialize L");
  }

  #[test]
  fn af() {
    let mut r = Registers::new();
    r.a = 0xAB;
    r.f = 0xCF;

    assert_eq!(r.af(), 0xABC0, "Get AF , last 4 bits are zeroed");
  }

  #[test]
  fn bc() {
    let mut r = Registers::new();
    r.b = 0xAB;
    r.c = 0xCF;

    assert_eq!(r.bc(), 0xABCF, "Get BC");
  }

  #[test]
  fn de() {
    let mut r = Registers::new();
    r.d = 0xAB;
    r.e = 0xCF;

    assert_eq!(r.de(), 0xABCF, "Get DE");
  }

  #[test]
  fn hl() {
    let mut r = Registers::new();
    r.h = 0xAB;
    r.l = 0xCF;

    assert_eq!(r.hl(), 0xABCF, "Get HL");
  }

  #[test]
  fn set_af() {
    let mut r = Registers::new();

    r.set_af(0xABCD);

    assert_eq!(r.a, 0xAB, "Set A");
    assert_eq!(r.f, 0xC0, "Set F, last 4 bits are zeroed");
  }

  #[test]
  fn set_bc() {
    let mut r = Registers::new();

    r.set_bc(0xABCD);

    assert_eq!(r.b, 0xAB, "Set B");
    assert_eq!(r.c, 0xCD, "Set C");
  }

  #[test]
  fn set_de() {
    let mut r = Registers::new();

    r.set_de(0xABCD);

    assert_eq!(r.d, 0xAB, "Set D");
    assert_eq!(r.e, 0xCD, "Set E");
  }

  #[test]
  fn set_hl() {
    let mut r = Registers::new();

    r.set_hl(0xABCD);

    assert_eq!(r.h, 0xAB, "Set H");
    assert_eq!(r.l, 0xCD, "Set L");
  }
}
