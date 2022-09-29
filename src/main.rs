#![no_std] // Rustの標準ライブラリをリンクしない
#![no_main] // Rustのエントリポイントを無効にする

use core::panic::PanicInfo;
mod vga_buffer;

// パニック時にこの関数が呼ばれる
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// OSのエントリポイントを独自の関数で上書き
// Cの呼び出し規約を使用することをコンパイラに伝えるために
// extern "C"として定義する
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world{}", "!");
    panic!("some panic message!");
    loop {}
}
