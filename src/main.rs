#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::{read_volatile, write_volatile}};

#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = main;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> ! {
    let rcc_gpio_en: *mut u32 = 0x4002_3830 as *mut u32;

    unsafe {
        write_peripheral_bit(rcc_gpio_en, 0, 1);
        write_peripheral_bit(rcc_gpio_en, 2, 1);
    }

    gpio_a_moder_write();
    gpio_a_otyper_write();
    gpio_a_pupdr_write();
    gpio_c_moder_write();

    /*loop {
        spin(1000);
        gpio_a_odr_write(1);
        spin(1000);
        gpio_a_odr_write(0);
    }*/

    loop {
        let v = get_user_in();
        if v == 0 {
            gpio_a_odr_write(1);
        }
        else {
            gpio_a_odr_write(0);
        }
    }
}

unsafe fn write_peripheral_bit(addr: *mut u32, bit_number: u8, data: u32) {
    let peripheral_base: usize = 0x4000_0000;
    let peripheral_alias_base: usize = 0x4200_0000;

    let alias_address: *mut u32 = (
        peripheral_alias_base +
        ((addr as usize - peripheral_base) * 32) +
        (bit_number as usize * 4)
    ) as *mut u32;

    unsafe {
        write_volatile(alias_address, data);
    }
}

unsafe fn read_peripheral_bit(addr: *const u32, bit_number: u8) -> u32 {
    let peripheral_base: usize = 0x4000_0000;
    let peripheral_alias_base: usize = 0x4200_0000;

    let alias_address: *const u32 = (
        peripheral_alias_base +
        ((addr as usize - peripheral_base) * 32) +
        (bit_number as usize * 4)
    ) as *const u32;

    unsafe {
        read_volatile(alias_address)
    }
}

fn gpio_a_moder_write() {
    unsafe {
        let addr = 0x4002_0000 as *mut u32;

        write_peripheral_bit(addr, 11, 0);
        write_peripheral_bit(addr, 10, 1);
    }
}

fn gpio_a_otyper_write() {
    unsafe {
        let addr = 0x4002_0004 as *mut u32;

        write_peripheral_bit(addr, 5, 0);
    }
}

fn gpio_a_pupdr_write() {
    unsafe {
        let addr = 0x4002_000c as *mut u32;

        write_peripheral_bit(addr, 11, 1);
        write_peripheral_bit(addr, 10, 0);
    }
}

fn gpio_a_odr_write(data: u32) {
    unsafe {
        let addr = 0x4002_0014 as *mut u32;

        write_peripheral_bit(addr, 5, data);
    }
}

fn gpio_c_moder_write() {
    unsafe {
        let addr = 0x4002_0800 as *mut u32;

        write_peripheral_bit(addr, 26, 0);
        write_peripheral_bit(addr, 27, 0);
    }
}

fn get_user_in() -> u32 {
    unsafe {
        read_peripheral_bit(0x4002_0810 as *const u32, 13)
    }
}

fn spin(f: u32) {
    unsafe {
        let i = 0x20000000 as *mut u32;
        write_volatile(i, 0);
        while read_volatile(i) < (1024 * f) {
            write_volatile(i, read_volatile(i) + 1);
        }
    }
}

#[panic_handler]
fn panic(_a: &PanicInfo) -> ! {
    loop {
        
    }
}