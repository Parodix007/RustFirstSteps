use traits::traits::Square;

use traits::traits::longest;

fn main() {
	let _costam = Square{
		side: 20,
	};

	let _longest = longest(String::from("dupa"), String::from("dupaaa"));

	println!("{:?}", _longest);
	//costam.calc_field();
}