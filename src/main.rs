#![feature(asm)]
#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn _start() -> u64 {
    static HELLO: &[u8] = b"Hello World!";
    debug(HELLO);
    exit(0)
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
