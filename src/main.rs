#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

mod vga_buffer;

static PRINT: &[u8] = b"Hello world!";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");

    unsafe { exit_qemu() }

    loop {}
}

unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);

    port.write(51);
}
