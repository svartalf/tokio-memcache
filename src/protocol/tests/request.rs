use protocol::{Request, Command};

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
fn test_request_add_serialization() {
    let mut request = Request::new(Command::Add);
    request.set_key(b"Hello");
    request.set_value(b"World");
    request.set_extras(&vec![0xde, 0xad, 0xbe, 0xef, 0x00, 0x00, 0x0e, 0x10]);

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