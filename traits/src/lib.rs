pub mod traits {
	//-------------------------
	// Cwiczenia na typach generycznych
	use std::fmt::Display;

	pub trait CalcField {
		fn calc_field(&self) {
			println!("Enter a valid values");
		}
	}

	#[derive(Debug)]
	pub struct Square<T> {
		pub side: T,
	}

	impl<T: Display> CalcField for Square<T>{
		fn calc_field(&self) {
			println!("Filed is : {}", &self.side);
		}
	}

	//-------------------------
	//Cwiczenia na lifetime

	pub fn longest(string1: String, string2: String) -> String {
		if string1.len() > string2.len() {
			string1
		} else {
			string2
		}
	}
}
