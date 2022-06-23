use serde::{Serialize, Deserialize};
use reqwest::{Error, blocking::Client};
use std::{io};

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
    let resp = client.get("https://jsonplaceholder.typicode.com/posts/4").send()?;
    let mut user: User = resp.json()?;

    user.body = clean_api_text_content(&user.body);
    user.title = clean_api_text_content(&user.title);

    Ok(user)
}

fn write_to_csv(user: User) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.serialize(user)?;
    wtr.flush()?;
    
    Ok(())
}


fn main() {

    let json_data = match get_json_api_data() {
        Ok(user) => user,
        Err(err) => panic!("Something went wrong upon getting the json data, {}", err),

    };
    println!("User got from API:\n {:#?}", json_data);
    println!("\nCSV line generated:\n");
    _ = write_to_csv(json_data);

}
