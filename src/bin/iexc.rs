use clap::{App, Arg};
use iexc;

fn main() {
    let matches = App::new("IEXC")
        .version("0.1")
        .author("Mikael L. <c.mikael.larsson@gmail.com>")
        .arg(
            Arg::new("API token")
                .short('a')
                .long("api-token")
                .value_name("TOKEN")
                .about("API token used for all requests to IEX endpoints")
                .takes_value(true),
        )
        .get_matches();

    if let Some(api_token) = matches.value_of("API token") {
        let _client = iexc::Client::new(api_token.to_string());
    } else {
        println!("No API token given");
    }
}
