use std::io;

#[derive(Debug)]

struct Person {
	name: String,
	last_name: String,
	age: u16,
	email: String,
}

struct Numbers(u16, u32, u8);

impl Person {
	fn set_person(name: String, last_name: String, age: u16, email: String) -> Person {
		Person {
			name,
			last_name,
			age,
			email,
		}
	}

	fn change_age(&mut self, value: u16) {
		self.age = value;
	}
}

fn main() {
	let mut _person = Person::set_person(String::from("Johe"), String::from("Doe"), 10, String::from("abc@gmail.com"));

	//println!("Przed metoda zmiany wieku: {:#?}", _person);

	_person.change_age(20);

    //println!("Po metodzie zmiany wieku: {:#?}", _person);

    let _numbers = Numbers(8,40,30);

    //println!("{}", numbers.0);

}
