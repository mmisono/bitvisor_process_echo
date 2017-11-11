#![crate_type = "staticlib"]
#![no_std]
#![feature(alloc)]
#![feature(global_allocator)]
#![feature(iterator_step_by)]
#![feature(inclusive_range_syntax)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate bitvisor_process_lib;

use bitvisor_process_lib::*;
use bitvisor_process_lib::mm::{Allocator,heap_init};

#[global_allocator]
static ALLOC: Allocator = Allocator;

static mut HEAP: [u8; 1024 * 1024] = [0; 1024 * 1024];

#[no_mangle]
pub extern "C" fn _start(_a1: i32, _a2: i32) -> i32 {
    let mut buf: [u8; 10] = [0; 10];

    unsafe{
        heap_init(&HEAP as *const _ as usize, HEAP.len());
    }
    println!("heap initialized");

    loop {
        print!("echo> ");
        let input = io::lineinput(&mut buf).unwrap();
        if input == "exit" {
            break;
        }
        println!("{}", input);

        if let Ok(n) = input.parse::<u32>() {
            let mut a = vec![2];
            for i in (3..=n).step_by(2) {
                if a.iter().filter(|&n| i % n == 0).take(1).count() == 0 {
                    print!("{} ", i);
                    a.push(i);
                }
            }
            println!{};
        }
    }
    syscalls::exitprocess(0);

    0
}
