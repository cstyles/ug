use std::io::{Read, Write};
use uuid::Uuid;
use Case::*;
use Format::*;

enum Case {
    Lowercase,
    Uppercase,
}

enum Format {
    Text,
    Binary,
}

fn main() {
    let (case, format) = parse_args();

    let namespace = Uuid::NAMESPACE_OID;
    let message = read_from_stdin();
    let message = message.trim_end();
    let uuid = Uuid::new_v5(&namespace, message.as_bytes());

    match (format, case) {
        (Text, Lowercase) => println!("{uuid:x}"),
        (Text, Uppercase) => println!("{uuid:X}"),
        (Binary, _) => {
            let mut stdout = std::io::stdout();
            let bytes = uuid.as_ref();
            match stdout.write(bytes) {
                Ok(bytes_written) => assert_eq!(bytes_written, bytes.len()),
                Err(err) => eprintln!("{err}"),
            }
        }
    }
}

fn read_from_stdin() -> String {
    if atty::is(atty::Stream::Stdin) {
        eprintln!("stdin is a tty. Please pipe something in.");
        std::process::exit(1);
    }

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    buffer
}

fn parse_args() -> (Case, Format) {
    let mut case = Lowercase;
    let mut format = Text;

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-l" | "--lowercase" => case = Lowercase,
            "-U" | "--uppercase" => case = Uppercase,
            "-t" | "--text" => format = Text,
            "-b" | "--binary" => format = Binary,
            _ => {
                eprintln!("Unrecognized option: {arg}");
                std::process::exit(1);
            }
        }
    }

    (case, format)
}
