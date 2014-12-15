use std::io;
fn main() {
    println!("*******/// GPA Calculator \\\\\\*******");
    println!("Developed by Apophis, Written in Rust\n\n *Note: this is based on Wyomissing's computing system(Simple GPA)\n\n");
    println!("Instructions: Type the uppercase letter grade then press enter\n enter `done` to calculate GPA")
    let mut stdin = io::stdin();
    let mut i = 1f32;
    let mut accum = 0f32;
    let mut reader = stdin.lock();
    let (A, B, C, D, F) = (4f32, 3f32, 2f32, 1f32, 0f32);
    for line in reader.lines() {
        let srt = line.unwrap();
        match srt.as_slice().trim() {
            "A" => {
                accum += A;
                println!("Class: {}, GPA Points= {}" , i , A)
            }
            "B" => {
                accum += B;
                println!("Class: {}, GPA Points= {}" , i , B)
            }
            "C" => {
                accum += C;
                println!("Class: {}, GPA Points= {}" , i , C)
            }
            "D" => {
                accum += D;
                println!("Class: {}, GPA Points= {}" , i , D)
            }
            "F" => {
                accum += F;
                println!("Class: {}, GPA Points= {}" , i , F)
            }
            "done" => {
                i += -1.00;
                println!("Calculated GPA: {}" , accum / i)
            }
            _ => println!("Input Invalid!"),
        }
        i += 1.00;
    }
}
