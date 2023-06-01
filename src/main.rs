use std::{fs, env, io::ErrorKind};

fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    let verifiable_path = match args.first() {
        Some(path) => path,
        None => "./"
    };

    match fs::metadata(verifiable_path) {
        Ok(_) => {
            let mut directories: Vec<String> = Vec::new();
            let paths = fs::read_dir(verifiable_path).unwrap();

            for path in paths {
                let document = path
                    .unwrap()
                    .file_name()
                    .into_string()
                    .unwrap();

                directories.push(document);
            }

            println!("{}", directories.join("   "));
        },
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("WARNING: The path does not exist");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}
