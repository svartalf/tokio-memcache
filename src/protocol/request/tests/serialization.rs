use serde_json;
use bytes::BytesMut;

use ::{Request, Command, command};

#[test]
fn test_get() {
    let mut request: Request<_, _, ()> = Request::build()
        .command(command::Get)
        .key(Some(b"Hello"))
        .finish();

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
    let mut result: Vec<u8> = Vec::new();
    let mut serializer = serde_json::Serializer::new( &mut result);
    request.to_writer(&serializer).unwrap();
//    assert_eq!(result, expected);
//    assert_eq!(request.len(), expected.len());
}
//
//#[test]
//fn test_delete() {
//    let mut request = Request::new(Command::Delete);
//    request.set_key(Some(b"Hello"));
//
//    let expected: Vec<u8> = vec![
//        0x80, 0x04, 0x00, 0x05,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x05,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x48, 0x65, 0x6c, 0x6c,
//        0x6f,
//    ];
//
//    let mut result: Vec<u8> = vec![];
//    request.to_writer(&mut result).unwrap();
//    assert_eq!(result, expected);
//    assert_eq!(request.len(), expected.len());
//}
//
//#[test]
//fn test_quit() {
//    let request = Request::new(Command::Quit);
//
//    let expected: Vec<u8> = vec![
//        0x80, 0x07, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//    ];
//
//    let mut result: Vec<u8> = vec![];
//    request.to_writer(&mut result).unwrap();
//    assert_eq!(result, expected);
//    assert_eq!(request.len(), expected.len());
//}
//
//#[test]
//fn test_noop() {
//    let request = Request::new(Command::Noop);
//
//    let expected: Vec<u8> = vec![
//        0x80, 0x0a, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//    ];
//
//    let mut result: Vec<u8> = vec![];
//    request.to_writer(&mut result).unwrap();
//    assert_eq!(result, expected);
//    assert_eq!(request.len(), expected.len());
//}
//
//#[test]
//fn test_version() {
//    let request = Request::new(Command::Version);
//
//    let expected: Vec<u8> = vec![
//        0x80, 0x0b, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//        0x00, 0x00, 0x00, 0x00,
//    ];
//
//    let mut result: Vec<u8> = vec![];
//    request.to_writer(&mut result).unwrap();
//    assert_eq!(result, expected);
//    assert_eq!(request.len(), expected.len());
//}
