use std::fs;
use std::num::Wrapping;
use std::str;

#[derive(Debug)]
pub enum HeaderSection {
    EntryPoint,
    NintendoLogo,
    Title,
    SGB,
    CartridgeType,
    ROMSize,
    RAMSize,
    LicCode,
    DestinationCode,
    Checksum,
}

#[derive(Debug, PartialEq)]
pub enum LicCode {
    NoLicense,
    Nintendo,
    Capcom,
    HotB,
    Jaleco,
    Coconuts,
    EliteSystems,
}

#[derive(Debug, PartialEq)]
pub enum DestinationCode {
    Japan,
    NonJapanese,
}

#[derive(Debug)]
pub enum CartridgeType {
    ROMOnly,
    MBC1,
    MBC1RAM,
    MBC1RAMBATTERY,
    MBC2,
}

pub struct Cart {
    content: Vec<u8>,
    title: String,
    rom_size: u8,
    ram_size: u8,
    cartridge_type: u8,
    lic_code: u8,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            content: vec![],
            title: String::from(""),
            rom_size: 0x00,
            ram_size: 0x00,
            cartridge_type: 0x00,
            lic_code: 0x00,
        }
    }

    pub fn title(self: &Cart) -> &str {
        self.title.trim_matches(char::from(0))
    }

    pub fn rom_size(self: &Cart) -> u16 {
        32 << self.rom_size
    }

    pub fn ram_size(self: &Cart) -> u8 {
        match self.ram_size {
            0x00 => 0,
            0x01 => 2,
            0x02 => 8,
            0x03 => 32,
            ram_size => panic!("cannot find RAM size {}", ram_size),
        }
    }

    pub fn checksum(self: &Cart) -> u8 {
        self.read_section(HeaderSection::Checksum)[0]
    }

    pub fn destination_code(self: &Cart) -> DestinationCode {
        match self.read_section(HeaderSection::DestinationCode)[0] {
            0x00 => DestinationCode::Japan,
            0x01 => DestinationCode::NonJapanese,
            code => panic!("destination code not supported: {}", code),
        }
    }

    pub fn calculate_checksum(self: &Cart) -> u16 {
        let mut x = Wrapping(0u16);

        for i in 0x0134..=0x14C {
            x = x - Wrapping(self.content[i] as u16) - Wrapping(1u16);
        }

        x.0 & 0xFF
    }

    pub fn cartridge_type(self: &Cart) -> CartridgeType {
        match self.cartridge_type {
            0x00 => CartridgeType::ROMOnly,
            0x01 => CartridgeType::MBC1,
            0x02 => CartridgeType::MBC1RAM,
            0x03 => CartridgeType::MBC1RAMBATTERY,
            0x05 => CartridgeType::MBC2,
            _ => panic!("cartridge type not implemented!"),
        }
    }

    pub fn lic_code(self: &Cart) -> LicCode {
        match self.lic_code {
            0x00 => LicCode::NoLicense,
            0x01 => LicCode::Nintendo,
            0x08 => LicCode::Capcom,
            0x09 => LicCode::HotB,
            0x0A => LicCode::Jaleco,
            0x0B => LicCode::Coconuts,
            lic_code => panic!("license code not implemented: {}", lic_code),
        }
    }

    pub fn read_file(self: &mut Cart, filename: &str) {
        let file = match fs::read(filename) {
            Ok(result) => result,
            Err(_) => panic!("File not found."),
        };

        self.content = file;
        self.title = String::from(str::from_utf8(self.read_section(HeaderSection::Title)).unwrap());
        self.rom_size = self.read_section(HeaderSection::ROMSize)[0];
        self.ram_size = self.read_section(HeaderSection::RAMSize)[0];
        self.cartridge_type = self.read_section(HeaderSection::CartridgeType)[0];
        self.lic_code = self.read_section(HeaderSection::LicCode)[0];
    }

    pub fn read_section(self: &Cart, section: HeaderSection) -> &[u8] {
        match section {
            HeaderSection::EntryPoint => &self.content[0x100..=0x103],
            HeaderSection::NintendoLogo => &self.content[0x104..=0x133],
            HeaderSection::Title => &self.content[0x134..=0x142],
            HeaderSection::SGB => &self.content[0x146..=0x146],
            HeaderSection::CartridgeType => &self.content[0x147..=0x147],
            HeaderSection::ROMSize => &self.content[0x148..=0x148],
            HeaderSection::RAMSize => &self.content[0x149..=0x149],
            HeaderSection::DestinationCode => &self.content[0x14A..=0x14A],
            HeaderSection::LicCode => &self.content[0x14B..=0x14B],
            HeaderSection::Checksum => &self.content[0x14D..=0x14D],
            section => panic!("header section not implemented: {:?}", section),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cart = Cart::new();

        assert_eq!(cart.content, vec![], "Initialize content");
        assert_eq!(cart.title, "", "Initialize title");
        assert_eq!(cart.rom_size, 0x00, "Initialize ROM size");
        assert_eq!(cart.ram_size, 0x00, "Initialize RAM size");
        assert_eq!(cart.cartridge_type, 0x00, "Initialize cartridge type");
        assert_eq!(cart.lic_code, 0x00, "Initialize license code");
    }

    #[test]
    fn read_section() {
        let mut cart = Cart::new();

        cart.read_file("test_roms/cpu_instrs.gb");

        assert_eq!(
            cart.read_section(HeaderSection::EntryPoint),
            vec![0x00, 0xC3, 0x37, 0x06],
            "Entry Point"
        );

        assert_eq!(
            cart.read_section(HeaderSection::NintendoLogo),
            vec![
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0xB, 0x03, 0x73, 0x00, 0x83, 0x00, 0xC,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
            ],
            "Nintendo Logo"
        );

        assert_eq!(cart.title(), "CPU_INSTRS", "Title");

        assert_eq!(
            cart.read_section(HeaderSection::SGB),
            vec![0x00],
            "SGB Flag"
        );

        assert_eq!(cart.rom_size, 0x01, "ROM Size");

        assert_eq!(cart.ram_size, 0x00, "RAM Size");

        assert_eq!(cart.cartridge_type, 0x01, "Cartridge Type");

        assert_eq!(cart.lic_code(), LicCode::NoLicense, "License Code");

        assert_eq!(
            cart.destination_code(),
            DestinationCode::Japan,
            "Destination Code"
        );

        assert_eq!(cart.checksum(), 0x3B, "Checksum");

        assert_eq!(cart.calculate_checksum(), 0x3B, "Checksum verify");
    }
}
