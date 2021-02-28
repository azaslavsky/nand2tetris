use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use vm::Translator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let output_path = &args[2];

    // Plays nicely regardless of whether the input is a single .vm file, or a directory containing
    // one or more of them.
    let mut input_files = vec![];
    let md = fs::metadata(input_path)
        .expect("input path not found");
    if md.is_file() {
        input_files.push(input_path.clone());
    } else {
        input_files = fs::read_dir(input_path)
            .expect("could not read directory")
            .into_iter()
            .filter_map(|file| {
                match file {
                    Ok(dir_entry) => Some(dir_entry
                        .path()
                        .to_str()
                        .expect("could not read directory entry")
                        .to_string()),
                    Err(_) => None,
                }
            })
            .collect();
    }

    // Filter out input files that don't have a ".vm" extension, and make sure that we have at
    // least one such input.  We also do a bit of a hack here: files called Sys.vm are expected to
    // contain the "Sys.init" function definition, though this is not strictly true.  If such a file
    // is present, we assume that the test script will not boot the system for us, and that we need
    // to run the "boot" VM command ourselves.
    let mut translator = Translator::new();
    input_files = input_files
        .into_iter()
        .filter(|file_path| {
            let path = Path::new(file_path);
            if "Sys.vm" == path.file_name().and_then(OsStr::to_str).unwrap_or("") {
                translator
                    .add_vm_file("boot\n")
                    .expect("could not add boot instruction");
            }
            "vm" == Path::new(file_path)
                .extension()
                .and_then(OsStr::to_str)
                .expect("file extension not readable")
        })
        .collect();
    if input_files.is_empty() {
        panic!("must specify a path containing at least one .vm file as input")
    }

    input_files
        .iter()
        .for_each(|file_path| {
            translator
                .add_vm_file(fs::read_to_string(file_path)
                .expect("could not read file")
                .as_str())
                .expect("translation failed");
        });

    let translated = translator.to_string();
    println!("\nout:\n{}", translated);
    fs::write(output_path, translated)
        .expect("could not write file");
}
