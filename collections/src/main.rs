#[derive(Debug)]
enum DiffrentTypes {
	Int(u16),
	Text(String),
	Bool(bool)
}
fn main() {
    let mut vec = vec![1, 2, 3];
    let mut outcome = Vec::new();

    println!("{:?}", vec);

    //vec.push(4);

    // println!("{:?}", vec);

    for item in &vec {
    	outcome.push(item * item);
    }

    //println!("{:?}", outcome);

    let number = &vec[0];

    vec.push(6);

    println!("{:?}", vec.get(100));

    for item in &mut vec {
    	*item *= *item;
    }

    println!("{:?}", vec);

    let difftypes = vec![DiffrentTypes::Int(10), DiffrentTypes::Bool(false), DiffrentTypes::Text(String::from("dupa"))];

    println!("{:?}", difftypes[0]);
}
