use iexc::http::{Client};

fn main() {
    let client = Client::new("httpbin.org".to_string());
    let response = client.get("/get").unwrap();
    println!("{}", response.body.trim_end());
}
