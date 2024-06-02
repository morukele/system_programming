use crate::{check_time, Clock};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use clap::{App, Arg};

pub fn run_clock_app() {
    let app = App::new("clock")
        .version("0.1")
        .about("Get and set the time")
        .after_help(
            "Note: UNIX timestamps are parsed as whole \
            seconds since 1st January 1970 0:00:00 UTC. \
            For more accuracy, use another format",
        )
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set", "check-ntp"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("standard")
                .takes_value(true)
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(
            Arg::with_name("datetime")
                .help("When <action> is 'set', apply <datetime>. Otherwise, ignore."),
        );

    let args = app.get_matches();

    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

    if action == "set" {
        let t_ = args.value_of("datetime").unwrap();

        let parser = match std {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };

        let err_msg = format!("Unable to parse {} according to {}", t_, std);
        let t = parser(t_).expect(&err_msg);

        Clock::set(t);
    } else if action == "check-ntp" {
        let offset = check_time().unwrap() as isize;

        let adjusted_ms_ = offset.signum() * offset.abs().min(200) / 5;
        let adjusted_ms = ChronoDuration::milliseconds(adjusted_ms_ as i64);

        let now: DateTime<Utc> = Utc::now() + adjusted_ms;

        Clock::set(now);
    }

    // Error handling
    let maybe_error = std::io::Error::last_os_error();
    let os_error_code = &maybe_error.raw_os_error();

    match os_error_code {
        Some(0) => (),
        Some(_) => eprintln!("Unable to set the time {:?}", maybe_error),
        None => (),
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        &_ => {
            unreachable!()
        }
    }
}
