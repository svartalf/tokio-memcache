/// Available protocol commands
///
/// Reference: https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped#command-opcodes

pub enum Command {
    Get = 0x00,
    Set = 0x01,
}
