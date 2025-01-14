use crate::packets::Packet;

#[test]
fn reqconn_with_path() {
    let before = Packet::Connection(super::Group::ReqConn {
        label: "hello".to_string().bytes().collect(),
        socket: Some(("socket_path".to_string(), vec![1, 2, 3, 4])),
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn reqconn_without_path() {
    let before = Packet::Connection(super::Group::ReqConn {
        label: "hello".to_string().bytes().collect(),
        socket: None,
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn apprconn() {
    let before = Packet::Connection(super::Group::ApprConn {
        echo: vec![1, 2, 3, 4],
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn rejconn() {
    let before = Packet::Connection(super::Group::RejConn {
        echo: vec![1, 2, 3, 4],
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}
