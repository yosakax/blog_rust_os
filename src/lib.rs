#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// なにこれ
#![reexport_test_harness_main = "test_main"]
// ABI呼び出し規約が不安定なためx86-interruptsを使うとエラーが出るので，
// それを強制的に使用するために以下を記述
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

// init_idtをmain.rsから間接的に呼び出す一般的なinit関数
// ここに書けばテストでも使用できるのでうれしい
pub fn init() {
    interrupts::init_idt();
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        // 関数の名前を出力
        serial_print!("{}...\t", core::any::type_name::<T>());
        // テスト
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    // println!("Running {} tests", tests.len());
    serial_println!("Running {} tests!", tests.len());
    for test in tests {
        test.run();
    }
    // qemuを閉じる
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// cargo test のときのエントリポイント
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

// test mode で使うパニックハンドラ
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
