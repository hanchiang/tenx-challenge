use std::env;
use std::io;
use std::process;
use std::fs::File;
use std::io::Read;

fn read_file(file_name: &str) ->  Result<String, io::Error> {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(e) => return Err(e)
    };
    Ok(buffer)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>, e.g. cargo run input.txt");
        process::exit(1);
    }

    let file_content = match read_file(&args[1]) {
        Ok(content) => content,
        Err(e) => {
            println!("Error encountered: {}\nExiting...", e);
            process::exit(1);
        }
    };
    println!("{:#?}", file_content);
}
