#![crate_type = "staticlib"]
#![no_std]

#[macro_use]
extern crate bitvisor_process_lib;

use bitvisor_process_lib::*;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start(_a1: i32, _a2: i32) -> i32 {
    let mut buf: [u8; 128] = [0; 128];
    loop {
        print!("echo> ");
        let input = io::lineinput(&mut buf).unwrap();
        if input == "exit" {
            break;
        }
        println!("{}", input);
    }
    syscalls::exitprocess(0);

    0
}
