#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let mut arr = [5, -3, 0, 3, 1, 2, -1, 8, 7];
    println!("before: {:?}",arr);
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    println!("after：{:?}",arr);
    0
}

