#![no_std] // Rustの標準ライブラリをリンクしない
#![no_main] // Rustのエントリポイントを無効にする
#![feature(custom_test_frameworks)]
#![test_runner(blog_rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_rust_os::println;
use core::panic::PanicInfo;

// OSのエントリポイントを独自の関数で上書き
// Cの呼び出し規約を使用することをコンパイラに伝えるために
// extern "C"として定義する
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Asumi Kana");
    // panic!("some panic message!");

    #[cfg(test)]
    test_main();

    loop {}
}

// パニック時にこの関数が呼ばれる
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // blog_rust_os::test_panic_handler(info)
    println!("{}", info);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
