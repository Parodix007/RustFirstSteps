use std::collections::HashMap;

#[derive(Debug)]
enum DiffrentTypes {
	Int(u16),
	Text(String),
	Bool(bool)
}
fn main() {
    let mut vec = vec![1, 2, 3];
    let mut outcome = Vec::new();

    //println!("{:?}", vec);

    //vec.push(4);

    // println!("{:?}", vec);

    for item in &vec {
    	outcome.push(item * item);
    }

    //println!("{:?}", outcome);

    let number = &vec[0];

    vec.push(6);

    //println!("{:?}", vec.get(100));

    for item in &mut vec {
    	*item *= *item;
    }

    //println!("{:?}", vec);

    let difftypes = vec![DiffrentTypes::Int(10), DiffrentTypes::Bool(false), DiffrentTypes::Text(String::from("dupa"))];

    //println!("{:?}", difftypes[0]);

    //Zadania z ksiazki

    let mut list_of_int = Vec::new();

    for item in 1..30{
        list_of_int.push(item);
    }

    let mut suma = 0;
    for item in &list_of_int{
        suma+=item;
    }
    //println!("Suma wszystkich wartosci: {:?}\nDlugosc tablicy: {:?}", suma, list_of_int.len());

    let mut powtorzenia_liczb = HashMap::new();

    let liczby = vec![1,2,3,1,4,4,3,5,6,7,6,4,5,7];

    for item in &liczby{
        let powtorzenie = powtorzenia_liczb.entry(item.to_string()).or_insert(0);
        *powtorzenie+=1;
    }

    //println!("{:?}", powtorzenia_liczb);

    let wyraz = "hallo";

    let mut litery = Vec::new();
    let mut nowy_wyraz = String::new();

    for item in wyraz.chars() {
        litery.push(item);
    }

    // println!("{:?}", litery);
    // println!("{:?}", litery.len());

    for item in (0..litery.len()).rev(){
        nowy_wyraz.push(litery[item]);
    }
    
    nowy_wyraz.push_str("ay");
    println!("{:?}", nowy_wyraz);


}
