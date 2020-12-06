use traits::traits::Square;

use traits::traits::longest;

use std::ptr; // function for checking if variables is pointing to the same memory location. ptr::eq

fn testing_ownership<T>(var: T) -> T {
	var
}

fn main() {
	let _costam = Square{
		side: 20,
	};

	let _string = String::from("dupa");
	let _string1 = String::from("dupaa");

	if longest(&_string, &_string1) == &_string { println!("{:?}", _string) } else { println!("{:?}", _string1) };

	let test;

	{
		let _test1 = 5;
		test = _test1;

		println!("{:?}", _test1);

	}

	let test2;
	let _test3 = 10;

	test2 = _test3;

	let mut owner = 20;

	{
		let test_owner = testing_ownership(owner); // I can pass a variable owner to this function and return a value from owner without any problem and after that owner is valid to use in the line 40 and 42. 

		// Now let check it out if variable owner and test_owner is pointing to the same memory location.
		let _mem = 10;
		let _test_mem = &_mem;
		let _test_mem1 = &_mem;
		println!("{:?}", (ptr::eq(_test_mem1, _test_mem))); 
	}

	owner = 10;

	println!("owner: {}", owner);

	//println!("test3: {:?}", _test3);
	//println!("test2: {:?}", test2);

	//println!("{:?}", &_test);
}