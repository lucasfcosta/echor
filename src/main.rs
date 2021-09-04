use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1")
        .author("Lucas da Costa <lucas@lucasfcosta.com>")
        .about("Prints whatever is given to it")
        .arg(
            Arg::with_name("input")
                .value_name("INPUT")
                .help("The input to print")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .help("Omits the newline character")
                .takes_value(false)
                .short("n"),
        )
        .get_matches();

    let ending = if matches.is_present("omit_newline") {
        ""
    } else {
        "\n"
    };

    let input = matches.values_of_lossy("input").unwrap();

    print!("{}{}", input.join(" "), ending)
}
