use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToTitleCase, ToTrainCase};
use std::fs::File;
use std::io::Read;
use std::io::Write;

// function to read in the text.txt file in the main directory
pub fn read_file() -> String {
    let mut file = File::open("./text.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
}

// function to convert the text to snake case and save as new file
pub fn convert_to_snake_case(text: String) {
    let snake_case = text.to_snake_case();
    let mut file = File::create("./snake_case.txt").expect("Unable to create file");
    file.write_all(snake_case.as_bytes())
        .expect("Unable to write data");
}

// function to convert the text to title case and save as new file
pub fn convert_to_title_case(text: String) {
    let title_case = text.to_title_case();
    let mut file = File::create("./title_case.txt").expect("Unable to create file");
    file.write_all(title_case.as_bytes())
        .expect("Unable to write data");
}

// function to convert the text to camel case and save as new file
pub fn convert_to_camel_case(text: String) {
    let camel_case = text.to_lower_camel_case();
    let mut file = File::create("./camel_case.txt").expect("Unable to create file");
    file.write_all(camel_case.as_bytes())
        .expect("Unable to write data");
}

// function to convert the text to train case and save as new file
pub fn convert_to_train_case(text: String) {
    let train_case = text.to_train_case();
    let mut file = File::create("./train_case.txt").expect("Unable to create file");
    file.write_all(train_case.as_bytes())
        .expect("Unable to write data");
}

// function to convert the text to kebab case and save as new file
pub fn convert_to_kebab_case(text: String) {
    let kebab_case = text.to_kebab_case();
    let mut file = File::create("./kebab_case.txt").expect("Unable to create file");
    file.write_all(kebab_case.as_bytes())
        .expect("Unable to write data");
}
