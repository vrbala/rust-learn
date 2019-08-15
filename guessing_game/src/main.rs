use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	
	let random_number = rand::thread_rng().gen_range(1, 101);
    
	println!("Guess the number!");
	
	loop {
		println!("Enter the number ... ");

		let mut guess = String::new(); // create a mutable location to store a string
	
		//& gets the reference of a value
		//by default, references are immutable
		//read_line needs a mutable reference of a value to fill it in
		io::stdin().read_line(&mut guess)
		.expect("Failed to read line");
		// return value of read_line is std::io::Result which is a 
		// enumeration with 2 variants (enumerations have specific values called variants)
		// Here, Result has Ok and Err variants to indicate success and failure of operation
		// .expect handles these variants by 1) returning value for Ok 2) printing the error and crashing on Err	
	
		println!("The entered value is {}", guess);
	
		// parse the string into number
		let guess : u32 = match guess.trim().parse() { // we are able to reuse `guess` here as Rust has shadowing
                    Ok(num) => num,
                    Err(_) => continue,
                };
			
		match guess.cmp(&random_number) {
			// following are called arms of match expression - made of pattern and code to run
			Ordering::Less => println!("You guessed less"),
			Ordering::Equal => {
				println!("You did it");
				break;
			},
			Ordering::Greater => println!("You guessed more"),
		}
	}
}
