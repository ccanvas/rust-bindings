use ccanvas_bindings::packets::*;

fn main() {
    println!(
        "{}",
        serde_json::to_string_pretty(&Packet::RejConn(RejConn)).unwrap()
    )
}
