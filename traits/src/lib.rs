pub mod traits {
	//-------------------------
	// Cwiczenia na typach generycznych
	use std::fmt::Display;

	pub trait CalcField { // Traits are using to group function of purpose for example functions that are returning a field of some figure 
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

	pub fn longest<'a>(string1: &'a String, string2: &'a String) -> &'a String {
		if string1.len() > string2.len() {
			string1
		} else {
			string2
		}
	}
}
