pub mod traits {
	
	pub trait CalcField {
		fn calc_field(&self) {
			println!("Enter a valid values");
		}
	}

	#[derive(Debug)]
	pub struct Square<T> {
		pub side: T,
	}

	impl<T> CalcField for Square<T>{
		fn calc_field(&self) {
			println!("Filed is : {}", (&self.side * &self.side));
		}
	}

	impl<T> Square<T> {
		pub fn echo(&self) {
			println!("ECHO");
		}
	}

	struct Rectangle<T, U> {
		side: T,
		side1: U,
	}
}
