//extern crate rand;

use std::io;
/*use std::cmp::Ordering;
use rand::Rng;
*/

// Function for Conv cel to fahr
/*fn conv_to_far(x: u32) -> u32 {
	(x * 9) / 5 + 32
}*/

fn n_of_fib(value: u32, n: u32) -> u32 {
	let mut outcome: u32 = value;

	for values in value..n {
		outcome = outcome + values;
	}

	outcome
}

fn main() {
	// GUESSING NUMBER GAME
	
	// ------------------
    // let sec_number = rand::thread_rng().gen_range(1,101);

    // loop {
    	
    // 	let mut input = String::new();
    // 	println!("Wprowadz liczbe: ");
	//     io::stdin().read_line(&mut input)
	//     					.expect("Enter a value");

	//     let input: u32 = match input.trim().parse() {
	//     	Ok(num) => num,
	//     	Err(_) => continue,
	//     };

	//     println!("Wprowadziles: {}", input);

	//     match input.cmp(&sec_number) {
	//     	Ordering::Less => println!("To small!"),
	//     	Ordering::Greater => println!("To much!"),
	//     	Ordering::Equal => {
	//     		println!("Big win!");
	//     		break;
	//     	},
	//     }

	// }
	// ------------------


	// Convert celsius to fahrentain
	// -----------------------------
	/*let mut input = String::new();

	println!("Enter temp in celsius");

	io::stdin().read_line(&mut input)
				.expect("Enter value!");

	let input: u32 = match input.trim().parse() {
		Ok(result) => result,
		Err(_) => 0
	};
	
	if input != 0 {
		println!("Your temperature in Fahrentine is {}", conv_to_far(input));
	} else {
		println!("You passed a wrong value");
	}*/
	// -----------------------------

	// Fib
	// ------------------
/*	let n1: u32 = 1;
	let mut n = String::new();

	println!("Enter n value of sequence: ");
	io::stdin().read_line(&mut n)
				.expect("Enter the number");

	let n = match n.trim().parse() {
		Ok(result) => result,
		Err(_) => 0,
	};


	if n > 0 {
		let n_value = n_of_fib(n1, n);
		println!("{} value of sequence: {}", n, n_value);
	}else{
		println!("Enter a valid number");
	}*/
	// ------------------

}

