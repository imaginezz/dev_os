#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dev_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
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

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use dev_os::memory;
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;

    println!("Hello {}!", "dev_os");
    dev_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    dev_os::hlt_loop();
}

entry_point!(kernel_main);

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
