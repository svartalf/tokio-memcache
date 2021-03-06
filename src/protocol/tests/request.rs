use std::default::Default;

#[cfg(feature = "nightly")]
use test::Bencher;

use protocol::{Request, Command, extras};

#[test]
fn test_request_get_serialization() {
    let mut request = Request::new(Command::Get);
    request.set_key(b"Hello");

    let expected: Vec<u8> = vec![
        0x80, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x48, 0x65, 0x6c, 0x6c,
        0x6f,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_request_set_serialization() {
    let mut request = Request::new(Command::Set);
    request.set_key(b"Hello");
    request.set_value(b"World");
    request.set_extras(extras::SetExtras::default());

    let expected: Vec<u8> = vec![
        0x80, 0x01, 0x00, 0x05,
        0x08, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x12,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x48, 0x65, 0x6c, 0x6c,
        0x6f, 0x57, 0x6f, 0x72,
        0x6c, 0x64,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_request_add_serialization() {
    let mut request = Request::new(Command::Add);
    request.set_key(b"Hello");
    request.set_value(b"World");
    request.set_extras(extras::SetExtras{
        flags: 0xdeadbeef,
        expiration: 3600,
    });

    let expected: Vec<u8> = vec![
        0x80, 0x02, 0x00, 0x05,
        0x08, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x12,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0xde, 0xad, 0xbe, 0xef,
        0x00, 0x00, 0x0e, 0x10,
        0x48, 0x65, 0x6c, 0x6c,
        0x6f, 0x57, 0x6f, 0x72,
        0x6c, 0x64,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_request_delete_serialization() {
    let mut request = Request::new(Command::Delete);
    request.set_key(b"Hello");

    let expected: Vec<u8> = vec![
        0x80, 0x04, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x48, 0x65, 0x6c, 0x6c,
        0x6f,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_request_incr_serialization() {
    let mut request = Request::new(Command::Increment);
    request.set_key(b"counter");
    request.set_extras(extras::IncrExtras{
        amount: 1,
        expiration: 3600,
        ..Default::default()
    });

    let expected: Vec<u8> = vec![
        0x80, 0x05, 0x00, 0x07,
        0x14, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x1b,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x0e, 0x10,
        0x63, 0x6f, 0x75, 0x6e,
        0x74, 0x65, 0x72,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_request_quit_serialization() {
    let request = Request::new(Command::Quit);

    let expected: Vec<u8> = vec![
        0x80, 0x07, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_request_noop_serialization() {
    let request = Request::new(Command::Noop);

    let expected: Vec<u8> = vec![
        0x80, 0x0a, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_request_version_serialization() {
    let request = Request::new(Command::Version);

    let expected: Vec<u8> = vec![
        0x80, 0x0b, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[cfg(feature = "nightly")]
#[bench]
fn bench_request_get_serialization(b: &mut Bencher) {
    let mut request = Request::new(Command::Get);
    request.set_key(b"Hello");

    b.iter(|| {
        let mut result: Vec<u8> = vec![];
        request.write(&mut result).unwrap();
    });
}