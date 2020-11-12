extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let sec_number = rand::thread_rng().gen_range(1,101);

    loop {
    	
    	let mut input = String::new();
    	println!("Wprowadz liczbe: ");
	    io::stdin().read_line(&mut input)
	    					.expect("Nie wprowadzono wartosci");

	    let input: u32 = match input.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => continue,
	    };

	    println!("Wprowadziles: {}", input);

	    match input.cmp(&sec_number) {
	    	Ordering::Less => println!("Za malo!"),
	    	Ordering::Greater => println!("Za duzo!"),
	    	Ordering::Equal => {
	    		println!("Trafione!");
	    		break;
	    	},
	    }

	}
}
