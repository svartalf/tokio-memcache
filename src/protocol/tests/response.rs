use tokio_core::io::EasyBuf;

use protocol::Response;

#[test]
fn test_response_get_from_easybuf() {
    let mut buf = EasyBuf::from(vec![
        0x81, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x09,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
        0xde, 0xad, 0xbe, 0xef,
        0x57, 0x6f, 0x72, 0x6c,
        0x64,
    ]);

    let response = Response::try_from(&mut buf).unwrap().unwrap();

    assert_eq!(*response.command(), 0x00);
    assert_eq!(*response.status(), 0x0000);
    assert_eq!(*response.data_type(), 0x00);

    assert_eq!(response.extras().unwrap(), [0xde, 0xad, 0xbe, 0xef]);
    assert!(response.key().is_none());
    assert_eq!(response.value().unwrap(), b"World");
}

#[test]
fn test_response_getk_from_easybuf() {
    // I dunno why, but memcached doc have a mistake here,
    // it says that instead of `0x0e` "total body length" byte
    // there should be a `0x09` which is looks totally wrong.
    let mut buf = EasyBuf::from(vec![
        0x81, 0x00, 0x00, 0x05,
        0x04, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x0e,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
        0xde, 0xad, 0xbe, 0xef,
        0x48, 0x65, 0x6c, 0x6c,
        0x6f, 0x57, 0x6f, 0x72,
        0x6c, 0x64,
    ]);

    let response = Response::try_from(&mut buf).unwrap().unwrap();

    assert_eq!(*response.command(), 0x00);
    assert_eq!(*response.status(), 0x0000);
    assert_eq!(*response.data_type(), 0x00);

    assert_eq!(response.extras().unwrap(), [0xde, 0xad, 0xbe, 0xef]);
    assert_eq!(response.key().unwrap(), b"Hello");
    assert_eq!(response.value().unwrap(), b"World");
}

#[test]
fn test_response_cas() {
    let mut buf = EasyBuf::from(vec![
        0x81, 0x02, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
    ]);

    let response = Response::try_from(&mut buf).unwrap().unwrap();

    assert_eq!(*response.command(), 0x02);
    assert_eq!(*response.status(), 0x0000);
    assert_eq!(*response.data_type(), 0x00);
    assert_eq!(*response.cas(), 0x0000000000000001);

    assert!(response.extras().is_none());
    assert!(response.key().is_none());
    assert!(response.value().is_none());
}

#[test]
fn test_response_incr_not_exists() {
    let mut buf = EasyBuf::from(vec![
        0x81, 0x05, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x08,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ]);

    let response = Response::try_from(&mut buf).unwrap().unwrap();

    assert_eq!(*response.command(), 0x05);
    assert_eq!(*response.status(), 0x0000);
    assert_eq!(*response.data_type(), 0x00);
    assert_eq!(*response.cas(), 0x0000000000000005);

    assert!(response.extras().is_none());
    assert!(response.key().is_none());
    assert_eq!(response.value().unwrap(), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn test_response_version() {
    /*
    Byte/     0       |       1       |       2       |       3       |
        /              |               |               |               |
       |0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|
       +---------------+---------------+---------------+---------------+
      0| 0x81          | 0x0b          | 0x00          | 0x00          |
       +---------------+---------------+---------------+---------------+
      4| 0x00          | 0x00          | 0x00          | 0x00          |
       +---------------+---------------+---------------+---------------+
      8| 0x00          | 0x00          | 0x00          | 0x05          |
       +---------------+---------------+---------------+---------------+
     12| 0x00          | 0x00          | 0x00          | 0x00          |
       +---------------+---------------+---------------+---------------+
     16| 0x00          | 0x00          | 0x00          | 0x00          |
       +---------------+---------------+---------------+---------------+
     20| 0x00          | 0x00          | 0x00          | 0x00          |
       +---------------+---------------+---------------+---------------+
     24| 0x31 ('1')    | 0x2e ('.')    | 0x33 ('3')    | 0x2e ('.')    |
       +---------------+---------------+---------------+---------------+
     28| 0x31 ('1')    |
       +---------------+
       Total 29 bytes (24 byte header, 5 byte body)

   Field        (offset) (value)
   Magic        (0)    : 0x81
   Opcode       (1)    : 0x0b
   Key length   (2,3)  : 0x0000
   Extra length (4)    : 0x00
   Data type    (5)    : 0x00
   Status       (6,7)  : 0x0000
   Total body   (8-11) : 0x00000005
   Opaque       (12-15): 0x00000000
   CAS          (16-23): 0x0000000000000000
   Extras              : None
   Key                 : None
   Value               : Textual string "1.3.1"
    */
    let mut buf = EasyBuf::from(vec![
        0x81, 0x0b, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x31, 0x2e, 0x33, 0x2e,
        0x31,
    ]);

    let response = Response::try_from(&mut buf).unwrap().unwrap();

    assert_eq!(*response.command(), 0x0b);
    assert_eq!(*response.status(), 0x0000);
    assert_eq!(*response.data_type(), 0x00);
    assert!(response.extras().is_none());
    assert!(response.key().is_none());
    assert_eq!(response.value().unwrap(), b"1.3.1");
}

#[test]
fn test_response_error_not_found() {
    let mut buf = EasyBuf::from(vec![
        0x81, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x09,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x4e, 0x6f, 0x74, 0x20,
        0x66, 0x6f, 0x75, 0x6e,
        0x64,
    ]);

    let response = Response::try_from(&mut buf).unwrap().unwrap();

    assert_eq!(*response.command(), 0x00);
    assert_eq!(*response.status(), 0x0001);
    assert_eq!(*response.data_type(), 0x00);
    assert!(response.extras().is_none());
    assert!(response.key().is_none());
    assert_eq!(response.value().unwrap(), b"Not found");
}