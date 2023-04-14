use std::io::Read;
use uuid::Uuid;

fn main() {
    let namespace = Uuid::NAMESPACE_OID;
    let message = read_from_stdin();
    let message = message.trim_end();
    let uuid = Uuid::new_v5(&namespace, message.as_bytes());
    println!("{uuid}");
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    buffer
}
