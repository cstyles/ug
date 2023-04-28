use std::io::{Read, Stdin, Write};
use uuid::Uuid;

use Case::*;
use Format::*;
use Version::*;

enum Case {
    Lowercase,
    Uppercase,
}

enum Format {
    Text,
    Binary,
}

enum Version {
    V4,
    V5,
}

fn main() {
    let (version, case, format) = parse_args();
    let uuid = generate_uuid(version.unwrap_or(V4));

    match (format, case) {
        (Text, Lowercase) => println!("{uuid:x}"),
        (Text, Uppercase) => println!("{uuid:X}"),
        (Binary, _) => print_binary_to_stdout(uuid),
    }
}

fn generate_uuid(version: Version) -> Uuid {
    match version {
        V4 => Uuid::new_v4(),
        V5 => {
            let namespace = Uuid::NAMESPACE_OID;
            let message = read_from_stdin();

            Uuid::new_v5(&namespace, message.as_bytes())
        }
    }
}

fn get_stdin() -> Option<Stdin> {
    atty::isnt(atty::Stream::Stdin).then(std::io::stdin)
}

fn read_from_stdin() -> String {
    let Some(mut stdin) = get_stdin() else {
        exit_with_error("stdin is a tty. Please pipe something in.")
    };

    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).unwrap();

    buffer
}

fn print_binary_to_stdout(uuid: Uuid) {
    let mut stdout = std::io::stdout();
    let bytes = uuid.as_ref();

    if let Err(err) = stdout.write_all(bytes) {
        exit_with_error(err);
    }
}

fn parse_args() -> (Option<Version>, Case, Format) {
    let mut case = Lowercase;
    let mut format = Text;
    let mut version = None;

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "v4" => version = Some(V4),
            "v5" => version = Some(V5),
            "-l" | "--lowercase" => case = Lowercase,
            "-U" | "--uppercase" => case = Uppercase,
            "-t" | "--text" => format = Text,
            "-b" | "--binary" => format = Binary,
            _ => exit_with_error(format!("Unrecognized option: {arg}")),
        }
    }

    (version, case, format)
}

fn exit_with_error(message: impl std::fmt::Display) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}
