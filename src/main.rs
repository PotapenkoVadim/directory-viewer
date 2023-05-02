use std::{fs, env, io::ErrorKind};

fn main() {
    let current_dir = "./";
    let not_found_message = "WARNING: The path does not exist";

    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    let verifiable_path = match args.first() {
        Some(path) => path,
        None => current_dir
    };

    match fs::metadata(verifiable_path) {
        Ok(_) => {
            let mut directories: Vec<String> = Vec::new();
            let paths = fs::read_dir(verifiable_path).unwrap();

            for path in paths {
                let document = path
                    .unwrap()
                    .path()
                    .display()
                    .to_string()
                    .replace(current_dir, "");

                directories.push(document);
            }

            println!("{}", directories.join("   "));
        },
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("{not_found_message}");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}
