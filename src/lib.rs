use std::{env, fs};

#[must_use]
pub fn read_file(folder: &str, name: &str) -> String {
    let cwd = env::current_dir().expect("To find current directory path");

    let filepath = cwd.join("src").join(folder).join(format!("{}.txt", name));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}
