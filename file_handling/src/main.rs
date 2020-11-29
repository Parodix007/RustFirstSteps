// Error handling with example of opening file
use std::io::Read;
use std::fs::File;
use std::io;

fn read_from_file(name_of_file: String) -> Result<String, io::Error> {
	let mut text_from_file1 = String::new();
	File::open(name_of_file)?.read_to_string(&mut text_from_file1)?;

	Ok(text_from_file1)
}

fn main() {
	// 1.
    let f = File::open("text.txt");
    let mut text_from_file = String::new();

    let mut file = match f {
    	Ok(file) => file,
    	Err(_) => panic!("Could not open a file."),
    };

    match file.read_to_string(&mut text_from_file) {
    	Ok(text) => text,
    	Err(_) => panic!("Could not read a file."),
    };

    println!("Hand writed match expressions for error handling: {:?}", text_from_file);

    // 2.
    let file_content = read_from_file(String::from("text.txt"));

    println!("Function with ? expression and Result as return: {:?}", file_content);

    // 3.
    let mut file1 = File::open("text.txt").expect("Could not open a file");
    let mut file_content1 = String::new();
    match file1.read_to_string(&mut file_content1) {
    	Ok(text) => text,
    	Err(_) => panic!("Could not read a file content"),
    };
    println!("expect with hand writed match to convert text content into file: {:?}", file_content1);

    // As long as I didint learn how to convert Ok into String third option is the best for me 
}
