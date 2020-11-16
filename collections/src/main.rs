fn main() {
    let mut vec = vec![1, 2, 3];
    let mut outcome: Vec<u32> = Vec::new();

    println!("{:?}", vec);

    vec.push(4);

    println!("{:?}", vec);

    for item in &vec {
    	outcome.push(item * item);
    }

    println!("{:?}", outcome);
}
