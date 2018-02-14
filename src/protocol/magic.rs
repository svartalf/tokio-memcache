enum_from_primitive! {
    /// Magic byte values for protocol packets.
    ///
    /// Reference: <https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped#magic-byte>
    #[repr(u8)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Magic {
        Request = 0x80,
        Response = 0x81,
    }
}
