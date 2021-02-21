use std::env;
use std::fs;
use vm::translate;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file)
        .expect("could not read file");
    let translated = translate(contents)
        .expect("translation failed");
    println!("\nout:\n{}", translated);
    fs::write(output_file, translated)
        .expect("could not write file");
}
