use std::io;

fn main() {
	println!("##########/// GPA Calculator \\\\\\############");
	println!("Developed by Karim Ellaisy, Written in Rust\n\n *Note: this is based on Wyomissing's computing system(Simple GPA)\n\n Enter your grades for your classes in order: ");
	let mut stdin = io::stdin();
	let mut i = 1i;
	let mut accum = 0i;
    let mut reader = stdin.lock();
    let (A,B,C,D,F) = (4i,3i,2i,1i,0i);
	for line in reader.lines() {
		let srt = line.unwrap();
		match srt.as_slice().trim() {
			"A" => {accum += A; println!("Class: {}, GPA Points= {}", i, A)},
			"B" => {accum += B; println!("Class: {}, GPA Points= {}", i, B)},
			"C" => {accum += C; println!("Class: {}, GPA Points= {}", i, C)},
			"D" => {accum += D; println!("Class: {}, GPA Points= {}", i, D)},
			"F" => {accum += F; println!("Class: {}, GPA Points= {}", i, F)},
			"done" =>{i += -1; println!("Calculated GPA: {}", accum / i)},
			_ => println!("Input Invalid!"),
		}
		i += 1;
	}
}

