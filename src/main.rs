#![no_std] // Rustの標準ライブラリをリンクしない
#![no_main] // Rustのエントリポイントを無効にする

use core::panic::PanicInfo;

// パニック時にこの関数が呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Asumi Kana\nhoge";
// OSのエントリポイントを独自の関数で上書き
// Cの呼び出し規約を使用することをコンパイラに伝えるために
// extern "C"として定義する
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // 特定のメモリの位置に値を代入しているだけ
            *vga_buffer.offset(i as isize * 2) = byte;
            // 出力文字の色指定
            *vga_buffer.offset(i as isize * 2 + 1) = 0xa;
        }
    }

    loop {}
}
