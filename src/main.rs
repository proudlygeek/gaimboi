mod cart;
mod cpu;
mod registers;

use std::env;
use cart::Cart;
use cpu::CPU;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cpu = CPU::new();
    let mut cart = Cart::new();
    cart.read_file(&args[1]);
    
    println!("{:?}", args);

    println!("Title: {:?}", cart.title());
    println!("ROM Size: {:?} KB", cart.rom_size());
    println!("RAM Size: {:?} KB", cart.ram_size());
    println!("Cartridge Type: {:?}", cart.cartridge_type());
    println!("License Code: {:?}", cart.lic_code());
    println!("Destination Code: {:?}", cart.destination_code());
    println!("Checksum: {:#02x} | Success: {}", cart.checksum(), cart.checksum() as u16 == cart.calculate_checksum());

    // loop {
    //     // cpu.execute_cycle();
    // }
}
