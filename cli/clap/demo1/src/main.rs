use clap::{Command, Arg};

fn main() {
    let matches = Command::new("demo1")
        .version("0.1.0")
        .author("Jean-Nicolas Boulay")
        .about("Simple Rust echo demo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
