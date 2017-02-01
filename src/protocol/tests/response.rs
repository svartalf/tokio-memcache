use test::Bencher;

use protocol::{Command, Status, DataType, Response};

#[test]
fn test_response_get() {
    let buf = vec![
        0x81, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x09,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
        0xde, 0xad, 0xbe, 0xef,
        0x57, 0x6f, 0x72, 0x6c,
        0x64,
    ];

    let response = Response::try_from(&buf).unwrap();

    assert_eq!(*response.command(), Command::Get);
    assert_eq!(*response.status(), Status::Ok);
    assert_eq!(*response.data_type(), DataType::RawBytes);

    assert_eq!(response.extras().unwrap(), [0xde, 0xad, 0xbe, 0xef]);
    assert!(response.key().is_none());
    assert_eq!(response.value().unwrap(), b"World");
}

#[test]
fn test_response_getk() {
    // I dunno why, but memcached doc have a mistake here,
    // it says that instead of `0x0e` "total body length" byte
    // there should be a `0x09` which is looks totally wrong.
    // Also, "opcode" byte is wrong, it should be a "getk" instead of "get".
    let buf = vec![
        0x81, 0x0c, 0x00, 0x05,
        0x04, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x0e,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
        0xde, 0xad, 0xbe, 0xef,
        0x48, 0x65, 0x6c, 0x6c,
        0x6f, 0x57, 0x6f, 0x72,
        0x6c, 0x64,
    ];

    let response = Response::try_from(&buf).unwrap();

    assert_eq!(*response.command(), Command::GetK);
    assert_eq!(*response.status(), Status::Ok);
    assert_eq!(*response.data_type(), DataType::RawBytes);

    assert_eq!(response.extras().unwrap(), [0xde, 0xad, 0xbe, 0xef]);
    assert_eq!(response.key().unwrap(), b"Hello");
    assert_eq!(response.value().unwrap(), b"World");
}

#[test]
fn test_response_cas() {
    let buf = vec![
        0x81, 0x02, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
    ];

    let response = Response::try_from(&buf).unwrap();

    assert_eq!(*response.command(), Command::Add);
    assert_eq!(*response.status(), Status::Ok);
    assert_eq!(*response.data_type(), DataType::RawBytes);
    assert_eq!(*response.cas(), 1u64);

    assert!(response.extras().is_none());
    assert!(response.key().is_none());
    assert!(response.value().is_none());
}

#[test]
fn test_response_incr_not_exists() {
    let buf = vec![
        0x81, 0x05, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x08,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let response = Response::try_from(&buf).unwrap();

    assert_eq!(*response.command(), Command::Increment);
    assert_eq!(*response.status(), Status::Ok);
    assert_eq!(*response.data_type(), DataType::RawBytes);
    assert_eq!(*response.cas(), 5u64);

    assert!(response.extras().is_none());
    assert!(response.key().is_none());
    assert_eq!(response.value().unwrap(), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn test_response_version() {
    let buf = vec![
        0x81, 0x0b, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x31, 0x2e, 0x33, 0x2e,
        0x31,
    ];

    let response = Response::try_from(&buf).unwrap();

    assert_eq!(*response.command(), Command::Version);
    assert_eq!(*response.status(), Status::Ok);
    assert_eq!(*response.data_type(), DataType::RawBytes);
    assert!(response.extras().is_none());
    assert!(response.key().is_none());
    assert_eq!(response.value().unwrap(), b"1.3.1");
}

#[test]
fn test_response_error_not_found() {
    let buf = vec![
        0x81, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x09,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x4e, 0x6f, 0x74, 0x20,
        0x66, 0x6f, 0x75, 0x6e,
        0x64,
    ];

    let response = Response::try_from(&buf).unwrap();

    assert_eq!(*response.command(), Command::Get);
    assert_eq!(*response.status(), Status::KeyNotFound);
    assert_eq!(*response.data_type(), DataType::RawBytes);
    assert!(response.extras().is_none());
    assert!(response.key().is_none());
    assert_eq!(response.value().unwrap(), b"Not found");
}

#[bench]
fn bench_parsing_response_get(b: &mut Bencher) {
    let buf = vec![
        0x81, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x09,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
        0xde, 0xad, 0xbe, 0xef,
        0x57, 0x6f, 0x72, 0x6c,
        0x64,
    ];

    b.iter(|| Response::try_from(&buf));
}
