use std::env;

fn config() -> (String, String) {
	let args: Vec<String> = env::args().collect();
	dbg!(args);

	return ("/Users/mahmudahmad/Downloads/tmp.yml".to_string(), "null".to_string());
}

fn main() {
	let input_file: String;
	let n:          String;

	(input_file, n) = config();
	println!("input_file: {input_file}, n: {n}");
}
