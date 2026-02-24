use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

// use `Result` return value to use ? operator in main()
// Box<dyn Error> is a trait object, means any kind of error
// Extended: main() return type can also be type which impl std::process::Termination trait
fn main() -> Result<(), Box<dyn Error>> {
    println!("use `Result` to handle recoverable errors");
    println!("two variants, Ok and Err");

    let readme_file_result = File::open("README.md");
    let readme_file = readme_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => File::create("README.md").unwrap(),
        _ => panic!("Problem opening the file: {error:?}"),
    });
    // quickly panic(or use unwrap())
    let readme_file = File::open("README.md").expect("open file error");

    // propagating error
    fn read_content_from_file() -> Result<String, io::Error> {
        let content_file_result = File::open("README.md");
        let mut content_file = match content_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut content = String::new();
        match content_file.read_to_string(&mut content) {
            Ok(_) => Ok(content),
            Err(e) => Err(e),
        }
    }
    fn read_content_from_file_simple() -> Result<String, io::Error> {
        // following lines are equal to `fs::read_to_string("README.md")`
        let mut content = String::new();
        // ? operator: Err will call from(`From` trait) to convert to io::Error
        File::open("README.md")?.read_to_string(&mut content)?;
        Ok(content)
    }
    fn last_char_of_first_line(text: &str) -> Option<char> {
        // ? operator can also be used in `Option`: return None
        // but `Option` can't be converted to `Result`, you should use ok_or method
        text.lines().next()?.chars().last()
    }

    Ok(()) // exit with value 0
}
