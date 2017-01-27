use tokio_core::io::{Codec, EasyBuf};

use codecs::text::TextCodec;


#[test]
fn test_encode_version() {
    let mut buf = EasyBuf::from("VERSION 1.4.32\r\n".to_string().into_bytes());
    let mut codec = TextCodec{};
    let response = codec.decode(&mut buf);
}