use serde::Deserialize;
use reqwest::{Error, blocking::Client};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}

fn get_json_api_data() -> Result<(), Error> {
    let client = Client::new();
    let resp = client.get("https://jsonplaceholder.typicode.com/posts/3").send()?;
    let user: User = resp.json()?;

    println!("User: {:#?}", user);

    Ok(())
}


fn main() {
    if let Err(err) = get_json_api_data() {
        eprintln!("Something went wrong, {}", err);
    }
}
