use std::thread;


struct Philosopher {
	name: String,
}


impl Philosopher {
	
	fn new(arg_name: &str) -> Philosopher {
		Philosopher {
			name: arg_name.to_string()
		}
	}

	fn eat(&self) {
		println!("{} is eating", self.name);

		thread::sleep_ms(1000);	// Sleep for 1000 milliseconds

		println!("{} is done eating", self.name);
	}
}


fn main() {
	println!("====================================");

	let philosophers = vec! [
		Philosopher::new("Judith Butler"),
		Philosopher::new("Gilles Deleuze"),
		Philosopher::new("Karl Marx"),
		Philosopher::new("Emma Goldman"),
		Philosopher::new("Michel Foucault"),
	];

	for p in &philosophers {
		p.eat();
	}

	println!("====================================");
}