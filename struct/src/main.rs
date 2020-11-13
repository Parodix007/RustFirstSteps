#[derive(Debug)]

struct Person {
	name: String,
	last_name: String,
	age: u16,
	email: String,
}

impl Person {
	fn set_person(_name: String, _last_name: String, age: u16, _email: String) -> Person {
		Person {
			name: String::from(_name),
			last_name: String::from(_last_name),
			age,
			email: String::from(_email)
		}
	}
}
fn main() {
	let _person = Person::set_person("John", "Doe", 10, "abc@gmail.com");

    println!("Hello, {:#?}", _person);
}
