pub fn connect() {
	println!("{}", (five() + 5));
}

fn five() -> u8 {
	5
}

pub fn calc_numbers(first_number: u8, secound_number: u8) -> Calc {
	Calc {
		first_number,
		secound_number
	}
}

pub struct Calc {
	first_number: u8,
	secound_number: u8,
}

impl Calc {
	pub fn add(&self) -> u8 {
		self.first_number + self.secound_number
	}

	fn sub(&self) -> u8 {
		self.first_number - self.secound_number
	}

	fn mult(&self) -> u8 {
		self.first_number * self.secound_number
	}
}