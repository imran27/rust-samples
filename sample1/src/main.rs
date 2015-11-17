extern crate rand;
use std::thread;
use std::io;


fn main() {
	let mut mynum = String::new();

	println!("Enter a number:");

	io::stdin().read_line(&mut mynum)
		.ok()
		.expect("Failed to read line!");

	let mynum: u64 = mynum.trim().parse()
		.ok()
		.expect("Not a Number");

	println!("Largest prime below {} is {}", mynum, get_prime(mynum) );
}


fn get_prime(num: u64) -> u64 {
	let mut retprime: u64 = 2;

	for i in 0..num {
		if is_prime(num - i) {
			retprime = num - i;
			break;
		}
	}

	retprime
}


fn is_prime(num: u64) -> bool {
	if (num == 1) | (num == 2) {
		return true;
	}

	for i in 2..num {
		if (num % i) == 0 {
			return false;
		}
	}

	true
}

