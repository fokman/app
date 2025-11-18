#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

// 这个函数会在硬件复位时被调用
#[entry]
fn main() -> ! {
    // 输出一条消息
    hprintln!("Hello, world!");
    
    // 退出QEMU
    debug::exit(debug::EXIT_SUCCESS);
    
    // 这个循环不会被执行到，但需要保留以满足返回类型要求
    loop {
        continue;
    }
}
