use clap::{App, Arg};

pub fn run() {
    let app = App::new("mget")
        .about("GET a webpage, manually")
        .arg(Arg::with_name("url").required(true))
        .arg(Arg::with_name("tap-device").required(true))
        .arg(
            Arg::with_name("dns-server").default_value("1.1.1.1")
        )
        .get_matches();
}
