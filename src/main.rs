#![feature(asm)]
#![no_std]
#![no_main]

#![feature(ptr_offset_from)]
extern crate core;

static mut PARGC: *const u64 = 0 as *const u64;
static mut ARGV: *const *const u8 = 0 as *const *const u8;

// Don't modify this function
#[no_mangle]
pub unsafe extern "C" fn _start() -> u64 {
    asm!("addi	sp,sp,16");
    asm!("mv $0, sp" : "=r"(PARGC) : : : "volatile");
    asm!("addi $0, sp, 8" : "=r"(ARGV) : : : "volatile");
    asm!("addi	sp,sp,-16");
    entry();
    exit(0)
}

#[no_mangle]
pub unsafe extern "C" fn entry() {
    let argc: u64 = *PARGC;
    let mut ptr: *const u8;
    let mut next_ptr: *const u8;
    let mut len;
    for i in 0..argc {
        ptr = *ARGV.offset(i as isize);
        next_ptr = *ARGV.offset(i as isize + 1);
        len = next_ptr.offset_from(ptr);
        let arg: &[u8] = core::slice::from_raw_parts(ptr, len as usize);
        debug(arg);
    }
}

#[no_mangle]
pub extern "C" fn syscall(_ar0: u64, _ar1: u64, _ar2: u64, _ar3: u64, _ar4: u64, _ar5: u64, _ar6: u64, _n: u64) -> u64 {
    let temp;
    unsafe {
        asm!("scall" : : : : "volatile");
        asm!("mv $0, a0" : "=r"(temp) : : : "volatile");
    }
    return temp;
}


#[no_mangle]
pub extern "C" fn exit(ret: i32) -> u64 {
    return syscall(ret as u64, 0, 0, 0, 0, 0, 0, 93);
}

#[no_mangle]
pub extern "C" fn debug(s: &[u8]) -> u64 {
    return syscall(s.as_ptr() as *const u8 as u64, 0, 0, 0, 0, 0, 0, 2177);
}

use core::panic::PanicInfo;
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(-1);
    loop {}
}

#[no_mangle]
pub extern fn abort() {
    exit(-1);
}
