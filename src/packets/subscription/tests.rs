use crate::packets::Packet;

#[test]
fn subscribe_with_nothing() {
    let before = Packet::Subscription(super::Group::Subscribe {
        channel: "channel".as_bytes().to_vec(),
        doas: None,
        echo: None,
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn subscribe_with_doas_only() {
    let before = Packet::Subscription(super::Group::Subscribe {
        channel: "channel".as_bytes().to_vec(),
        doas: Some("doas".as_bytes().to_vec()),
        echo: None,
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn subscribe_with_echo_only() {
    let before = Packet::Subscription(super::Group::Subscribe {
        channel: "channel".as_bytes().to_vec(),
        doas: None,
        echo: Some("echo".as_bytes().to_vec()),
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn subscribe_with_all() {
    let before = Packet::Subscription(super::Group::Subscribe {
        channel: "channel".as_bytes().to_vec(),
        doas: Some("doas".as_bytes().to_vec()),
        echo: Some("echo".as_bytes().to_vec()),
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn unsubscribe_with_nothing() {
    let before = Packet::Subscription(super::Group::Unsubscribe {
        channel: "channel".as_bytes().to_vec(),
        doas: None,
        echo: None,
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn unsubscribe_with_doas_only() {
    let before = Packet::Subscription(super::Group::Unsubscribe {
        channel: "channel".as_bytes().to_vec(),
        doas: Some("doas".as_bytes().to_vec()),
        echo: None,
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn unsubscribe_with_echo_only() {
    let before = Packet::Subscription(super::Group::Unsubscribe {
        channel: "channel".as_bytes().to_vec(),
        doas: None,
        echo: Some("echo".as_bytes().to_vec()),
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn unsubscribe_with_all() {
    let before = Packet::Subscription(super::Group::Unsubscribe {
        channel: "channel".as_bytes().to_vec(),
        doas: Some("doas".as_bytes().to_vec()),
        echo: Some("echo".as_bytes().to_vec()),
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn subscribed() {
    let before = Packet::Subscription(super::Group::Subscribed {
        echo: "echo".as_bytes().to_vec(),
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn unsubscribed() {
    let before = Packet::Subscription(super::Group::Unsubscribed {
        echo: "echo".as_bytes().to_vec(),
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn notfound() {
    let before = Packet::Subscription(super::Group::NotFound {
        echo: "echo".as_bytes().to_vec(),
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}

#[test]
fn denied() {
    let before = Packet::Subscription(super::Group::Denied {
        echo: "echo".as_bytes().to_vec(),
    });
    let bytes = before.to_bytes();
    let after = Packet::from_bytes(&bytes).unwrap();

    assert_eq!(
        super::Group::try_from(before).unwrap(),
        super::Group::try_from(after).unwrap()
    );
}
