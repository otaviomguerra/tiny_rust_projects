use reqwest::{blocking::Client, Error};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::path::Path;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}

fn clean_api_text_content(text: &str) -> String {
    text.replace("\n", "")
}

fn get_json_api_data() -> Result<User, Error> {
    let client = Client::new();
    let resp = client
        .get("https://jsonplaceholder.typicode.com/posts/2")
        .send()?;
    let mut user: User = resp.json()?;

    user.body = clean_api_text_content(&user.body);
    user.title = clean_api_text_content(&user.title);

    Ok(user)
}

fn open_file_in_correct_mode(filename: &str) -> (File, bool) {
    if Path::new(filename).exists() {
        let file_object = OpenOptions::new()
            .append(true)
            .open(filename)
            .expect("Unable to open file");
        (file_object, true)
    } else {
        let file_object = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)
            .expect("Unable to open file");
        (file_object, false)
    }

}

fn select_file_writer(file_exists: bool, file_obj: File) -> csv::Writer<File>{
    if file_exists {
        csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file_obj)
    } else {
        csv::Writer::from_writer(file_obj)
    }
}

fn write_to_csv(user: User) -> Result<(), Box<dyn std::error::Error>> {
    let (file, file_exists) = open_file_in_correct_mode("data.csv");
    let mut wtr = select_file_writer(file_exists,file);

    wtr.serialize(user)?;
    wtr.flush()?;

    Ok(())
}

fn main() {
    let json_data = match get_json_api_data() {
        Ok(user) => user,
        Err(err) => panic!("Something went wrong upon getting the json data, {}", err),
    };

    println!("User got from the API:\n {:#?}", json_data);
    _ = write_to_csv(json_data);

    println!("Successfully wrote API data to csv!");
}
