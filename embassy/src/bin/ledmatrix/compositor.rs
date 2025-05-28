use max7219::connectors::Connector;
use max7219::DataError;

pub enum WriteMode {
    OVERRIDE,
    BLEND
}

pub struct Compositor {
    data: [[u8; 8]; 4]
}

impl Compositor {
    pub fn new() -> Self {
        Compositor {
            data: [
                [0; 8],
                [0; 8],
                [0; 8],
                [0; 8],
            ]
        }
    }

    pub fn clear(&mut self) {
        self.data = [[0; 8]; 4];
    }

    // made to mirror direct max7219 API access
    pub fn write_raw(&mut self, num: usize, screen: &[u8; 8], mode: WriteMode) {
        match mode {
            WriteMode::BLEND => {
                for i in 0..8 {
                    self.data[num][i] |= screen[i];
                }
            },
            WriteMode::OVERRIDE => {
                self.data[num] = *screen;
            }
        }
    }

    pub fn shift_right(&mut self) {
        for i in 0..8 {
            self.data[3][i] >>= 1;
            self.data[3][i] |= (0b1 & self.data[2][i]) << 7;
        }
        for i in 0..8 {
            self.data[2][i] >>= 1;
            self.data[2][i] |= (0b1 & self.data[1][i]) << 7;
        }
        for i in 0..8 {
            self.data[1][i] >>= 1;
            self.data[1][i] |= (0b1 & self.data[0][i]) << 7;
        }
        for i in 0..8 {
            self.data[0][i] >>= 1;
        }
    }
    pub fn shift_left(&mut self) {
        for i in 0..8 {
            self.data[0][i] <<= 1;
            self.data[0][i] |= 0b1 & (self.data[1][i] >> 7);
        }
        for i in 0..8 {
            self.data[1][i] <<= 1;
            self.data[1][i] |= 0b1 & (self.data[2][i] >> 7);
        }
        for i in 0..8 {
            self.data[2][i] <<= 1;
            self.data[2][i] |= 0b1 & (self.data[3][i] >> 7);
        }
        for i in 0..8 {
            self.data[3][i] <<= 1;
        }
    }

    pub fn write_bit(&mut self, x: usize, y: usize, value: bool) {
        let row: u8 = 1 << (7 - (x % 8));
        let screen = x / 8;

        self.data[screen][y] |= row;
    }

    // takes maximum screen size to simplify, it's just 32 bytes anyway.
    // first row of first screen is byte 0, first row of 2nd screen is byte 1
    // 2nd row of first screen is byte 4 and so on

    pub fn blit(&mut self, xoff: usize, yoff: usize, xs: usize, ys: usize, data: &[u8; 8]) {
        for x in 0..xs {
            for y in 0..ys {
                if data[y] & (1 << (7 - (x % 8))) != 0 {
                    self.write_bit(x + xoff, y + yoff, true);
                }
            }
        }
    }
}

pub fn write_out<CONN: Connector>(compositor: &Compositor, display: &mut max7219::MAX7219<CONN>) -> Result<(), DataError> {
    for i in 0..compositor.data.len() {
        display.write_raw(i, &compositor.data[i])?;
    }
    Ok(())
}
