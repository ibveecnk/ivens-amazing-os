#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod interrupts;
pub mod vga_buffer;

/// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    {
        test_main();
        loop {}
    }

    println!("Hello World{}", "!");

    loop {}
}

pub fn init() {
    interrupts::init_idt();
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("=== Ivens Amazing OS Test Runner ===");
    println!();
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    println!("-> All tests passed :)");
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion ");
    assert_eq!(2, 1);
    println!("[ok]");
}
