use max7219::connectors::Connector;
use max7219::DataError;

pub struct Compositor {
    data: [[u8; 8]; 4]
}

impl Compositor {
    pub fn new() -> Self {
        Compositor {
            data: [
                [0,0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0,0],
            ]
        }
    }

    // made to mirror direct max7219 API access
    pub fn write_raw(&mut self, num: usize, row: &[u8; 8]) {
        self.data[num] = *row;
    }
}

pub fn write_out<CONN: Connector>(compositor: &Compositor, display: &mut max7219::MAX7219<CONN>) -> Result<(), DataError> {
    for i in 0..compositor.data.len() {
        display.write_raw(i, &compositor.data[i])?;
    }
    Ok(())
}