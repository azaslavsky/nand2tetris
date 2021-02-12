use std::env;
use std::fs;
use asm::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("could not read file");
    let parsed = Parser::new(contents)
        .commands()
        .iter()
        .fold(String::new(), |out, command| {
            let num = command.num();
            out + format!("{:016b}\n", num).as_str()
        });

    fs::write(output_file, parsed).expect("could not write file");
}
