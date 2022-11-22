use std::fs;

const STATIC_DIR: &str = "src/dataprovider/github/test/static";

pub fn read_file(path: &str) -> String {
    let path = format!("{}/{}", STATIC_DIR, path);
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    contents
}
