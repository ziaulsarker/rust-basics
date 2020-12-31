#![allow(dead_code)]

mod sh;

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
fn main() {
    let test_s = String::from("this is a sentence");
    println!("{:?} is in bytes", &test_s.as_bytes());

    let temp = convert_temp(32, true);
    println!("its {} outside", temp);
    println!("10th fib number {}", get_nth_fib_num(10));

    sh::stack_and_heap();
    
    let mut s = String::from("hello");
    s.push_str(" world");

    println!("{}", s);
}

fn convert_temp(mut temp: i32, cal_fah: bool) -> i32 {
    if !cal_fah {
        temp = (temp - 32) * (5 / 9);
    } else {
        temp = (temp * 9 / 5) + 32;
    }

    return temp;
}

fn get_nth_fib_num(nth_num: u32) -> u32 {
    match nth_num {
        0 => 0,
        1 => 1,
        _ => get_nth_fib_num(nth_num - 1) + get_nth_fib_num(nth_num - 2),
    }
}
