#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dev_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use dev_os::task::{executer::Executer, keyboard, Task};
use dev_os::{allocator, memory, println};
use x86_64::VirtAddr;

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

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello {}!", "dev_os");
    dev_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    let mut executer = Executer::new();
    executer.spawn(Task::new(example_task()));
    executer.spawn(Task::new(keyboard::print_keypresses()));
    executer.run();

    // dev_os::hlt_loop();
}

entry_point!(kernel_main);

async fn async_number() -> u32 {
    22
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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
