#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dev_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use dev_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    dev_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dev_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}!", "dev_os");

    dev_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    dev_os::hlt_loop();
}

#[cfg(test)]
mod tests {
    use dev_os::{serial_print, serial_println};
    #[test_case]
    fn trival_assertion() {
        serial_print!("trival assertion... ");
        assert_eq!(1, 1);
        serial_println!("[ok]");
    }

    // #[test_case]
    // fn trival_assertion_panic() {
    //     serial_print!("trival assertion_panic... ");
    //     assert_eq!(1, 0);
    //     serial_println!("[panic]");
    // }

    // #[test_case]
    // fn trival_assertion_loop() {
    //     serial_print!("trival assertion_loop... ");
    //     serial_println!("[loop]");
    //     loop {}
    // }
}
