use ccanvas_bindings::packets::{connection::ReqConn, Packet};

fn main() {
    let bytes = Packet::to_bytes(Packet::Connection(
        ccanvas_bindings::packets::connection::Group::ReqConn(ReqConn {
            label: "ti".to_string(),
            socket: Some("test".to_string()),
        }),
    ));

    dbg!(&bytes);
    dbg!(Packet::from_bytes(&bytes));
}
