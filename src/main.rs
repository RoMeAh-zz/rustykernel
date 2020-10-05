#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(standalone_binary::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use standalone_binary::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    standalone_binary::init();

    #[cfg(test)]
    test_main();

    stack();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    standalone_binary::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
