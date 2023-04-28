use std::io::{Read, Stdin, Write};
use uuid::{Uuid, Version};
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
    let (version, case, format) = parse_args();
    let uuid = generate_uuid(version.unwrap_or(Version::Random));

    match (format, case) {
        (Text, Lowercase) => println!("{uuid:x}"),
        (Text, Uppercase) => println!("{uuid:X}"),
        (Binary, _) => print_binary_to_stdout(uuid),
    }
}

fn generate_uuid(version: Version) -> Uuid {
    match version {
        Version::Random => Uuid::new_v4(),
        Version::Sha1 => {
            let namespace = Uuid::NAMESPACE_OID;
            let message = read_from_stdin();

            Uuid::new_v5(&namespace, message.as_bytes())
        }
        version => exit_with_error(format!("Unsupported UUID version: {version:?}")),
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
            "v4" => version = Some(Version::Random),
            "v5" => version = Some(Version::Sha1),
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
