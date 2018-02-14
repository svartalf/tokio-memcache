enum_from_primitive! {
    /// Reserved for future use.
    ///
    /// Reference: <https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped#data-types>
    #[repr(u8)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum DataType {
        RawBytes = 0x00,
    }
}
