fn main() {
	let mut number_one : String = String::new();
	let mut number_two : String = String::new();

	println!("Type first number: ");

	std::io::stdin().read_line(&mut number_one).unwrap();

	println!("Type second number: ");

	std::io::stdin().read_line(&mut number_two).unwrap();

	let number_one_int : u8 = number_one.trim().parse().unwrap();

	let number_two_int : u8 = number_two.trim().parse().unwrap(); 

	println!("{}", number_one_int + number_two_int);
}
