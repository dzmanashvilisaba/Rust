use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
	let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
		if error.kind() == ErrorKind::NotFound {
			File::create("hello.txt").unwrap_or_else(|error| {
				panic!("Problem creating the file: {:?}", error);
			})
		} else {
			panic!("Problem opening the file: {:?}", error);
		}
	});
}

fn read_username_from_file() -> Result<String, std::io::Error> {
	let username_file_result = File::open("hello.txt");
	let mut username_file = match username_file_result {
		Ok(file) => file,
		Err(e) => return Err(e),
	};
	let mut username = String::new();
	match username_file.read_to_string(&mut username) {
		Ok(_) => Ok(username),
		Err(e) => Err(e),
	}
}
