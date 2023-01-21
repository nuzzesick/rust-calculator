fn main() {
	let mut number_one : String = String::new();
	let mut number_two : String = String::new();
	let mut operator : String = String::new();

	println!("Type first number: ");

	std::io::stdin().read_line(&mut number_one).unwrap();

	println!("Type second number: ");

	std::io::stdin().read_line(&mut number_two).unwrap();

	println!("Choose an operator (+ or -): ");

	std::io::stdin().read_line(&mut operator).unwrap();

	if operator.trim() != "+" && operator.trim() != "-" {
		println!("Invalid operator!");
		std::process::exit(1);
	}

	let number_one_int : i32 = number_one.trim().parse().unwrap();

	let number_two_int : i32 = number_two.trim().parse().unwrap(); 

	if operator.trim() == "+" {
		println!("{}", number_one_int + number_two_int);
	} else {
		println!("{}", number_one_int - number_two_int);
	}
}
