	//Program pyta uzytkownika o 5 informacj nastepnie zapisuje je w HashMapie. Uzytkownik dostaje pytanie o nazwe infomracji ktora wprowadza oraz o sama infomracje. Nastepnie wyswietla sie HashMap i program pyta czy wszystko jest poprawne. Jezeli uzytkownik wprowadza jeszcze raz dane

use std::io;
use std::collections::HashMap;
use termion;
fn make_questions(){
	let mut dane = HashMap::new();
	for _ in 0..6 {
		print!("{}",termion::clear::All);
		let mut nazwa_informacji = String::new();
		let mut informacja = String::new();
		println!("Wprowadz nazwe dla infomracji ktora wprowadzasz: ");
		io::stdin().read_line(&mut nazwa_informacji)
			.expect("Nie wprowadzono nazwy informacji");
		println!("Wprowadz informacje: ");
		io::stdin().read_line(&mut informacja)
			.expect("Nie wprowadzono informacji");

		dane.insert(nazwa_informacji, informacja);
	}

	println!("Z funkcji: {:?}", dane);
}
fn main() {
	make_questions()
}
