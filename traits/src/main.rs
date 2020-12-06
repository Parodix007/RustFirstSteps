use traits::traits::Square;

use traits::traits::longest;

fn main() {
	let _costam = Square{
		side: 20,
	};

	let _string = String::from("dupa");
	let _string1 = String::from("dupaa");

	println!("{:?}", _string) if longest(&_string, &_string1) == _string else println!("{:?}", _string1);
	//costam.calc_field();
}