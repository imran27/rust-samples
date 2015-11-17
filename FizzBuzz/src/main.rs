/**
 * @file main.rs
 * 
 * 
 * Sun Nov 15 20:56:35 IST 2015
 *
 */


use std::io;


fn main() {
    let mut in_str = String::new();

    let mut is_num: bool = true;

    println! ("FizzBuzz example:");

    io::stdin().read_line(&mut in_str)
        .ok()
        .expect("Line not read");

    let in_str: u64 = in_str.trim().parse()
        .ok()
        .expect("Enter a number");

    if (in_str % 3) == 0 {
        print! ("Fizz");
        is_num = false;
    }

    if (in_str % 5) == 0 {
        print! ("Buzz");
        is_num = false;
    }

    if is_num {
        println! ("{}", in_str);
    } else {
        println! ("");
    }
}
