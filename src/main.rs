use std::io::{IsTerminal, Read, Stdin, Write};
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
        (Some(_), Some(V4)) => exit_with_error("Received piped input but v4 doesn't need input."),
        _ => Uuid::new_v4(),
    };

    print_uuid(uuid, format);
}

/// Read a UUID from stdin, either as bytes or as a string.
fn read_uuid_from_stdin(mut stdin: Stdin) -> Uuid {
    let mut buffer = Vec::with_capacity(40);

    if let Err(err) = stdin.read_to_end(&mut buffer) {
        let message = format!("Encountered error while reading UUID from input:\n'{err}'");
        exit_with_error(message);
    }

    // If we read exactly 16 bytes, return them as a UUID.
    if let Ok(uuid) = Uuid::from_slice(&buffer) {
        return uuid;
    };

    // Otherwise, interpret the input as a string-formatted UUID.
    let Ok(string) = String::from_utf8(buffer) else {
        exit_with_error("Input was not valid UTF-8.");
    };

    match Uuid::parse_str(string.trim_end()) {
        Ok(uuid) => uuid,
        Err(err) => {
            let message = format!("UUID read from input was invalid:\n'{err}'");
            exit_with_error(message);
        }
    }
}

/// Create a UUID v5 by hashing bytes read from stdin.
fn generate_v5(mut stdin: Stdin) -> Uuid {
    let namespace = Uuid::NAMESPACE_OID;
    let mut bytes = vec![];
    stdin.read_to_end(&mut bytes).unwrap();

    Uuid::new_v5(&namespace, &bytes)
}

fn print_uuid(uuid: Uuid, format: Format) {
    match format {
        Lowercase => print_text(format!("{uuid:x}")),
        Uppercase => print_text(format!("{uuid:X}")),
        Binary => print_binary_to_stdout(uuid),
    }
}

/// Returns `None` if stdin is a TTY. Otherwise returns a handle to stdin.
fn get_stdin() -> Option<Stdin> {
    let stdin = std::io::stdin();
    match stdin.is_terminal() {
        true => None,
        false => Some(stdin),
    }
}

/// Print a trailing newline if stdout is a TTY.
/// Otherwise (e.g., if piped) omit the trailing newline.
fn print_text(output: String) {
    match std::io::stdout().is_terminal() {
        true => println!("{output}"),
        false => print!("{output}"),
    }
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
    println!(
        include_str!("../help.txt"),
        program_name = std::env::args().next().unwrap()
    );
    std::process::exit(0);
}

fn exit_with_error(message: impl std::fmt::Display) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}
