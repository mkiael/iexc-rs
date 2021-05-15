use clap::{App, Arg};
use iexc;

fn main() {
    let matches = App::new("IEXC")
        .version("0.1")
        .author("Mikael L. <c.mikael.larsson@gmail.com>")
        .arg(
            Arg::new("api token")
                .short('a')
                .long("api-token")
                .required(true)
                .value_name("TOKEN")
                .about("API token used for all requests to IEX endpoints")
                .takes_value(true),
        )
        .arg(
            Arg::new("sandbox")
                .short('s')
                .long("sandbox")
                .about("Use sandbox endpoint"),
        )
        .get_matches();

    let endpoint = if matches.is_present("sandbox") {
        iexc::Endpoint::Sandbox
    } else {
        iexc::Endpoint::Production
    };

    let api_token = matches.value_of("api token").unwrap();

    let client = iexc::Client::new(endpoint, api_token.to_string());
    let latest_price_aapl = client.get_latest_price("aapl");
    println!("Latest price for Apple: {} $", latest_price_aapl.unwrap());
}
