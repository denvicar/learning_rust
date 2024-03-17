pub struct Crc32 {
    computing_table: [u32;256],
    crc: u32,
}

impl Crc32 {
    pub fn new() -> Crc32 {
        Crc32 { computing_table: make_crc_table(), crc: 0xffffffff }
    }

    fn update_crc(&mut self, buf: &[u8]) -> u32 {
        for byte in buf {
            let idx = (self.crc ^ (*byte as u32)) & 0xff;
            self.crc = self.computing_table[idx as usize] ^ (self.crc >> 8)
        }

        self.crc
    }

    pub fn crc(&mut self, buf: &[u8]) -> u32 {
        self.update_crc(buf) ^ 0xffffffff
    }
}

fn make_crc_table() -> [u32;256] {
    let mut c:u32;
    let mut crc_table = [0;256];

    for i in 0..256 {
        c = i;
        for _ in 0..8 {
            if c & 1 == 1 {
                c = 0xedb88320u32 ^ (c >> 1);
            } else {
                c = c >> 1;
            }
        }
        crc_table[i as usize] = c;
    }

    crc_table
}