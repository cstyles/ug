use std::io::{Read, Stdin, Write};
use uuid::Uuid;

use Format::*;
use Version::*;

enum Format {
    Lowercase,
    Uppercase,
    Binary,
}

enum Version {
    V4,
    V5,
}

fn main() {
    let (version, format) = parse_args();
    let stdin = get_stdin();

    let uuid = match (stdin, version) {
        (Some(stdin), None) => read_uuid_from_stdin(stdin),
        (Some(stdin), Some(V5)) => generate_v5(stdin),
        (None, Some(V5)) => exit_with_error("stdin is a tty. Please pipe something in."),
        _ => Uuid::new_v4(),
    };

    print_uuid(uuid, format);
}

fn read_uuid_from_stdin(mut stdin: Stdin) -> Uuid {
    let mut bytes = [0; 16];
    stdin.read_exact(&mut bytes).unwrap();

    Uuid::from_bytes(bytes)
}

fn generate_v5(stdin: Stdin) -> Uuid {
    let namespace = Uuid::NAMESPACE_OID;
    let bytes = read_from_stdin(stdin);

    Uuid::new_v5(&namespace, &bytes)
}

fn print_uuid(uuid: Uuid, format: Format) {
    match format {
        Lowercase => println!("{uuid:x}"),
        Uppercase => println!("{uuid:X}"),
        Binary => print_binary_to_stdout(uuid),
    }
}

fn get_stdin() -> Option<Stdin> {
    atty::isnt(atty::Stream::Stdin).then(std::io::stdin)
}

fn read_from_stdin(mut stdin: Stdin) -> Vec<u8> {
    let mut buffer = vec![];
    stdin.read_to_end(&mut buffer).unwrap();

    buffer
}

fn print_binary_to_stdout(uuid: Uuid) {
    if let Err(err) = std::io::stdout().write_all(uuid.as_ref()) {
        exit_with_error(err);
    }
}

fn parse_args() -> (Option<Version>, Format) {
    let mut format = Lowercase;
    let mut version = None;

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "v4" => version = Some(V4),
            "v5" => version = Some(V5),
            "-l" | "--lowercase" => format = Lowercase,
            "-U" | "--uppercase" => format = Uppercase,
            "-b" | "--binary" => format = Binary,
            _ => exit_with_error(format!("Unrecognized option: {arg}")),
        }
    }

    (version, format)
}

fn exit_with_error(message: impl std::fmt::Display) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}
