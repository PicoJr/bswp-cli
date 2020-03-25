use clap::{App, Arg};

const VERSION: &str = "0.1.0";

pub fn get_app() -> App<'static, 'static> {
    App::new("bswp-cli")
        .version(VERSION)
        .author("PicoJr")
        .about("Swap bytes using patterns and masks")
        .arg(
            Arg::with_name("pattern")
                .takes_value(true)
                .multiple(true)
                .min_values(1)
                .required(true)
                .short("e")
                .help("pattern: <value>,<mask>,<periodicity>,<offset>"),
        )
        .arg(
            Arg::with_name("input")
                .takes_value(true)
                .required(false)
                .short("i")
                .help("input path (if not provided STDIN is used instead)"),
        )
        .arg(
            Arg::with_name("output")
                .takes_value(true)
                .required(false)
                .short("o")
                .help("output path (if not provided STDOUT is used instead)"),
        )
}
