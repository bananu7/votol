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
}

pub fn write_out<CONN: Connector>(compositor: &Compositor, display: &mut max7219::MAX7219<CONN>) -> Result<(), DataError> {
    for i in 0..compositor.data.len() {
        display.write_raw(i, &compositor.data[i])?;
    }
    Ok(())
}
