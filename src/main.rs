/*
from io import BytesIO

class DiskIO(BytesIO):
    def read(self, size=-1):
        value = super().read(size)

        if size != -1 and len(value) < size:
            raise EOFError(
                f"Tried to read {size} bytes, but got {len(value)} bytes instead"
            )

        return value

    def read_byte(self):
        byte = self.read(1)
        return byte[0]

    def read_loc(self):
        loc = self.read(2)
        return tuple(loc)

    def read_word(self):
        word = self.read(2)
        return word[0] + word[1] * 256

    def read_until_null(self):
        line = b""

        byte = self.read(1)
        while byte != b"\0":
            line += byte
            byte = self.read(1)

        return line

    def skip(self, size):
        self.read(size)
*/
/*read_word
read_until_null
seek
read
tell
*/
use bytes::Bytes;

#[derive(Debug)]
struct MemoryBytes {
    bytes: Bytes,
    loc: usize,
}

impl MemoryBytes {
    fn new() -> MemoryBytes {
        MemoryBytes {
            bytes: Bytes::from(&b"Hello world"[..]),
            loc: 0,
        }
    }
}

trait BytesIO {
    fn read(&mut self, kind: ReadType) -> Bytes;

    fn read_word(&mut self) -> u16;
    fn read_until_null(&mut self) -> Result<Bytes, std::io::Error>;
    fn read_until_end(&mut self) -> Bytes;
    fn seek(&mut self, to: usize);
    fn tell(&self) -> usize;
}

enum ReadType {
    UntilEnd,
    Amount(usize)
}

impl BytesIO for MemoryBytes {
    fn read(&mut self, kind: ReadType) -> Bytes {
        let old_loc = self.loc;
        match kind {
            ReadType::UntilEnd => {
                self.loc = self.bytes.len();
                self.bytes.slice(old_loc .. self.bytes.len() - old_loc)
            }
            ReadType::Amount(to_read) => {
                self.loc = old_loc + to_read;
                self.bytes.slice(old_loc .. to_read)
            }
        }
    }

    fn read_word(&mut self) -> u16 {
        let bigendian = self.read(ReadType::Amount(2));
        bigendian[0] as u16 + bigendian[1] as u16 * 256
    }

    fn read_until_null(&mut self) -> Result<Bytes, std::io::Error> {
        let old_loc = self.loc;
        let mut i = 0;
        for x in self.bytes.split_off(old_loc) {
            if x == 0 {
                return Ok(self.bytes.split_to(i));
            }
            i += 1;
        }
        Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
    }

    fn read_until_end(&mut self) -> Bytes {
        self.read(ReadType::UntilEnd)
    }

    fn seek(&mut self, to: usize) {
        self.loc = to;
    }

    fn tell(&self) -> usize {
        self.loc
    }
}

fn main() {
    let mut b = MemoryBytes::new();
    println!("Hello, world! {:?}", b);
    b.seek(5);
    println!("Hello, world! {:?}", b);
}
