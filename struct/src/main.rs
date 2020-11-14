#[derive(Debug)]

struct Person {
	name: String,
	last_name: String,
	age: u16,
	email: String,
}

impl Person {
	fn set_person(name: String, last_name: String, age: u16, email: String) -> Person {
		Person {
			name,
			last_name,
			age,
			email,
		}
	}

	fn change_values(&mut self, type: String, value: String) -> RetType {
		let _value = match value.trim().parse() {
			Ok(result) => result,
			Err(_)
		}
	}
}
fn main() {
	let _person = Person::set_person(String::from("Johe"), String::from("Doe"), 10, String::from("abc@gmail.com"));

    println!("Hello, {:#?}", _person);
}
