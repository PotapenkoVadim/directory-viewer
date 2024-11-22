use std::{fs, env, io::ErrorKind};

fn main() {
    let result = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .first()
        .map_or_else(|| {Some("./")}, |p| {Some(p)})
        .map(|path| {
            fs::metadata(path).map_or_else(|err| {
                if err.kind() == ErrorKind::NotFound {"WARNING: The path does not exist".to_string()}
                else {panic!("Unexpected error: {:?}", err);}
            }, |_| {
                let mut directories: Vec<String> = Vec::new();
                let paths = fs::read_dir(path).unwrap();

                for path in paths {
                    let document = path
                        .unwrap()
                        .file_name()
                        .into_string()
                        .unwrap();
    
                    directories.push(document);
                }

                format!("{}", directories.join("   "))
            })
        })
        .unwrap();

    println!("{}", result);
}
