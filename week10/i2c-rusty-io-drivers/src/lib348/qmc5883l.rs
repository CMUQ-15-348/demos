use crate::lib348::i2c::I2C1;

// I2C address of the compass
const ADDRESS: u8 = 0x0D;

// Register addresses
const REG_XOUT_LSB: u8 = 0x00;
const REG_STATUS: u8 = 0x06;
const REG_CR1: u8 = 0x09;
const REG_CR2: u8 = 0x0A;
const REG_SR_PERIOD: u8 = 0x0B;

pub struct QMC5883L<'a> {
    i2c: &'a I2C1,
}

impl<'a> QMC5883L<'a> {
    pub fn new(i2c: &'a I2C1) -> Self {
        QMC5883L { i2c }
    }

    pub fn init(&self) {
        // Initialize the I2C peripheral
        self.i2c.write(ADDRESS, REG_CR2, 0x80); // Reset the sensor
        self.i2c.write(ADDRESS, REG_SR_PERIOD, 0x01); // Set the period based on the datasheet
        self.i2c.write(ADDRESS, REG_CR1, 0x11); // Set the sensor in continuous mode (from the datasheet)
    }

    pub fn read(&self) -> (i16, i16, i16) {
        // If you request 6 bytes starting at REG_XOUT_LSB, you will get all the readings at once.
        let mut data = [0u8; 6];
        self.i2c.read(ADDRESS, REG_XOUT_LSB, &mut data);

        // Combine the LSB and MSB into a single 16-bit value
        let x = ((data[1] as i16) << 8) | (data[0] as i16);
        let y = ((data[3] as i16) << 8) | (data[2] as i16);
        let z = ((data[5] as i16) << 8) | (data[4] as i16);

        // // Read the X value
        // let mut bytes = [0u8, 0u8];
        // self.i2c.read(ADDRESS, REG_XOUT_LSB, &mut bytes);
        // let x = (bytes[1] as i16) << 8 | bytes[0] as i16;
        // //info!("X: {}", x);

        // // Read the Y value
        // let mut bytes = [0u8, 0u8];
        // self.i2c.read(ADDRESS, REG_XOUT_LSB + 2, &mut bytes);
        // let y = (bytes[1] as i16) << 8 | bytes[0] as i16;
        // //info!("Y: {}", y);

        // // Read the Z values
        // let mut bytes = [0u8, 0u8];
        // self.i2c.read(ADDRESS, REG_XOUT_LSB + 4, &mut bytes);
        // let z = (bytes[1] as i16) << 8 | bytes[0] as i16;

        (x, y, z)
    }

    pub fn data_available(&self) -> bool {
        let mut status = [0u8; 1];
        self.i2c.read(ADDRESS, REG_STATUS, &mut status);
        // Check the data ready bit (bit 0)
        (status[0] & 0x01) != 0
    }
}
