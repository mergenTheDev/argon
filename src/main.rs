#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Oha!");
    println!("Ciddi");
    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop{}
}
