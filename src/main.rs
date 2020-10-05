#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(standalone_binary::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use standalone_binary::println;
use bootloader::{BootInfo, entry_point};
use standalone_binary::memory;
use x86_64::{structures::paging::MapperAllSizes, VirtAddr};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    standalone_binary::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
                0xb8000,
                0x201008,
                0x0100_0020_1a10,
                boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    standalone_binary::hlt_loop();
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
