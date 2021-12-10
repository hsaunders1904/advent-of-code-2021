extern crate clap;

pub mod cli {
    use clap::{App, Arg};

    pub struct InputArgs {
        pub file_path: String,
    }

    pub fn parse_args() -> InputArgs {
        let matches = App::new("Advent of code day 1")
            .version("0.0.1")
            .arg(
                Arg::with_name("data_file")
                    .value_name("DATA_FILE")
                    .takes_value(true)
                    .help("Text file containing data - one integer per line")
                    .required(true),
            )
            .get_matches();

        InputArgs {
            file_path: matches.value_of("data_file").unwrap().to_owned(),
        }
    }
}
