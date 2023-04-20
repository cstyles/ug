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

/// Parse a UUID from bytes read from stdin.
fn read_uuid_from_stdin(mut stdin: Stdin) -> Uuid {
    let mut bytes = [0; 16];
    stdin.read_exact(&mut bytes).unwrap();

    Uuid::from_bytes(bytes)
}

/// Create a UUID v5 by hashing bytes read from stdin.
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

/// Returns `None` if stdin is a TTY. Otherwise returns a handle to stdin.
fn get_stdin() -> Option<Stdin> {
    atty::isnt(atty::Stream::Stdin).then(std::io::stdin)
}

fn read_from_stdin(mut stdin: Stdin) -> Vec<u8> {
    let mut buffer = vec![];
    stdin.read_to_end(&mut buffer).unwrap();

    buffer
}

fn print_binary_to_stdout(uuid: Uuid) {
    if let Err(err) = std::io::stdout().write_all(uuid.as_bytes()) {
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
            "-u" | "--uppercase" => format = Uppercase,
            "-b" | "--binary" => format = Binary,
            "-h" | "--help" => print_help(),
            _ => exit_with_error(format!("Unrecognized option: {arg}")),
        }
    }

    (version, format)
}

fn print_help() -> ! {
    let program_name = std::env::args().next().unwrap();

    println!("USAGE: {program_name} [OPTIONS] [VERSION]");
    println!();
    println!("Arguments:");
    println!("  [VERSION]        Which version of UUID to use. Options are v4 (default) and v5.");
    println!();
    println!("Options:");
    println!("  -l, --lowercase  Output UUID using lowercase letters (the default).");
    println!("  -u, --uppercase  Output UUID using uppercase letters.");
    println!("  -b, --binary     Output UUID in binary format.");

    std::process::exit(0);
}

fn exit_with_error(message: impl std::fmt::Display) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}
