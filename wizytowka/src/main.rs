	//Program pyta uzytkownika o 5 informacj nastepnie zapisuje je w HashMapie. Uzytkownik dostaje pytanie o nazwe infomracji ktora wprowadza oraz o sama infomracje. Nastepnie wyswietla sie HashMap i program pyta czy wszystko jest poprawne. Jezeli uzytkownik wprowadza jeszcze raz dane

use std::io;
use std::collections::HashMap;
use termion;

fn get_data() -> HashMap<String, String>{
	let mut dane = HashMap::new();
	for _ in 0..2 {
		print!("-------\n");
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
	
	dane
}

fn handle_data_error(mut hash_to_handle: HashMap<String,String>) {
	let mut vec_danych: Vec<(String,String)> = Vec::new();
		

	for (key,value) in &hash_to_handle{
		let mut dane_informacji = String::new();
		let mut czy_zgadza = String::new();

		println!("POPRAWA DANYCH------------", );
		println!("{}: {}", key.trim(), value.trim());
		println!("Zgadza sie?");
		io::stdin().read_line(&mut czy_zgadza)
					.expect("Nie wprowadzono infomracji czy obiekt sie zgadza");

		println!("CZY ZGADZA SIE Z POPRZEDNIEJ ITERACJI{:?}", czy_zgadza);
		if czy_zgadza.trim().to_ascii_uppercase() == "NIE"{
			println!("Podaj nowa wartosc:");
			io::stdin().read_line(&mut dane_informacji)
						.expect("Nie podano danych");

			let klucz = key;
			vec_danych.push((klucz.to_string(), dane_informacji.to_string()));
		}
	}

	for item in &vec_danych{
		hash_to_handle.insert((item.0).to_string(), (item.1).to_string());
	}
	
	validate(hash_to_handle);
}

fn validate(hash_to_valid: HashMap<String, String>){
	let mut zgoda = String::new();

	println!("WALIDACJA DANYCH-------");
	print!("{}", termion::clear::All);
	for (key, value) in &hash_to_valid{
		println!("{}: {}", key.trim(), value.trim());
	}
	println!("Czy dane sie zgadzaja?");
	io::stdin().read_line(&mut zgoda)
				.expect("Wprowadz informacje o poprawnosci");

	if zgoda.trim().to_ascii_uppercase() == "TAK" {
		println!("Dziekujemy za informacje");
		println!("To twoje dane: ");

		for (key, value) in &hash_to_valid{
			println!("{}: {}", key.trim(), value.trim());
		}
	}else{
		handle_data_error(hash_to_valid)
	}
}
fn iterate_over_hash(hash_to_iter:HashMap<String,String>){
	println!("-------------");
	for (key, item) in &hash_to_iter {
		println!("{}: {}", key.trim(), item.trim());
	}
}

fn main() {
	let _dane = get_data();
	validate(_dane);
}
