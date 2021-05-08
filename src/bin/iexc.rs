use iexc::http::{Client};

fn main() {
    let client = Client::new("google.com".to_string());
    let response = client.get().unwrap();
    println!("{}", response.body.trim_end());
}
