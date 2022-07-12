use std::{env, fs::File, io::Read, path::PathBuf};

pub fn read_file(filename: &PathBuf) -> String {
    let path = env::current_dir().unwrap().as_path().join(filename);
    let display = path.display();

    // println!("Current Path: {:?}", path);

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why)
    }
    s
}
