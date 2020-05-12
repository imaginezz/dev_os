#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dev_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

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
    println!("Hello {}!", "dev_os");
    dev_os::init();

    use dev_os::memory;
    use x86_64::VirtAddr;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
    use dev_os::allocator;

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    let heap_value = Box::new(11);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 1..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("current reference count is {} now", Rc::strong_count(&cloned_reference));

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
