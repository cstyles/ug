use std::io::Read;
use uuid::Uuid;

enum Case {
    Lowercase,
    Uppercase,
}

fn main() {
    let case = parse_args();

    let namespace = Uuid::NAMESPACE_OID;
    let message = read_from_stdin();
    let message = message.trim_end();
    let uuid = Uuid::new_v5(&namespace, message.as_bytes());

    match case {
        Case::Lowercase => println!("{uuid:x}"),
        Case::Uppercase => println!("{uuid:X}"),
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

fn parse_args() -> Case {
    let mut case = Case::Lowercase;

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-l" | "--lowercase" => case = Case::Lowercase,
            "-U" | "--uppercase" => case = Case::Uppercase,
            _ => {}
        }
    }

    case
}
