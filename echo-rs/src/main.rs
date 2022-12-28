use clap::{Command, Arg, ArgAction};

fn main() {
    let matches = Command::new("echo-rs")
        .version("0.1.0")
        .author("Otavio Guerra <otaviomguerra@gmail.com>")
        .about("Rust version of echo command")
        .arg(
            Arg::new("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
            .required(false)
            .short('n')
            .help("Do not print newline")
            .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text: Vec<&str> = matches.get_many::<String>("text")
                                 .unwrap()
                                 .map(|s| s.as_str()).collect();
    //println!("{:#?}", text);
    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
