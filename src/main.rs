use ccanvas_bindings::packets::*;

fn main() {
    let bytes = Packet::to_bytes(Packet::Connection(connection::Group::ReqConn {
        label: "ti".to_string(),
        socket: Some("test".to_string()),
    }));

    dbg!(&bytes);
    dbg!(Packet::from_bytes(&bytes));
}
